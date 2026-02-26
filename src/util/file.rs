pub mod paths {
    use camino::Utf8PathBuf;
    use std::io;

    pub fn ensure_paths_exist() -> io::Result<()> {
        if let Err(err) = std::fs::create_dir_all(ssbusync()) {
            println!(
                "[ssbusync][config] failed to create config directory {}: {}",
                ssbusync(),
                err
            );
            return Err(err);
        }
        Ok(())
    }

    pub fn ssbusync() -> Utf8PathBuf {
        Utf8PathBuf::from("sd:/ultimate/ssbusync")
    }

    pub fn ssbusync_config() -> Utf8PathBuf {
        ssbusync().join("ssbusync.toml")
    }

    pub fn ssbusync_disablers() -> Utf8PathBuf {
        ssbusync().join("disablers.toml")
    }
}

pub mod config {
    use super::paths;
    use crate::SsbuSyncConfig;
    use serde::{Deserialize, Serialize};
    use std::collections::BTreeMap;
    use std::io;

    const DEFAULT_PROFILE_VERSION: f32 = 1.0;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum DefaultProfileState {
        Created,
        Loaded,
    }

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(default)]
    struct ConfigFile {
        #[serde(rename = "SsbuSync")]
        ssbusync: BTreeMap<String, ProfileEntry>,
    }

    #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
    #[serde(default)]
    struct ProfileEntry {
        profile_version: f32,
        #[serde(flatten)]
        config: SsbuSyncConfig,
    }

    impl Default for ProfileEntry {
        fn default() -> Self {
            Self {
                profile_version: DEFAULT_PROFILE_VERSION,
                config: SsbuSyncConfig::default(),
            }
        }
    }

    fn io_err(err: impl std::fmt::Display) -> io::Error {
        io::Error::new(io::ErrorKind::InvalidData, err.to_string())
    }

    fn load_file(path: &camino::Utf8PathBuf) -> io::Result<ConfigFile> {
        if !path.exists() {
            return Ok(ConfigFile::default());
        }
        let data = match std::fs::read_to_string(path.as_std_path()) {
            Ok(data) => data,
            Err(err) => {
                println!(
                    "[ssbusync][config] failed reading {}: {}",
                    path,
                    err
                );
                return Err(err);
            }
        };

        match toml::from_str(&data) {
            Ok(parsed) => Ok(parsed),
            Err(err) => {
                println!(
                    "[ssbusync][config] failed parsing TOML {}: {}",
                    path,
                    err
                );
                Err(io_err(err))
            }
        }
    }

    fn write_file(path: &camino::Utf8PathBuf, data: &ConfigFile) -> io::Result<()> {
        let toml = match toml::to_string_pretty(data) {
            Ok(toml) => toml,
            Err(err) => {
                println!(
                    "[ssbusync][config] failed serializing TOML {}: {}",
                    path,
                    err
                );
                return Err(io_err(err));
            }
        };

        if let Err(err) = std::fs::write(path.as_std_path(), toml) {
            println!(
                "[ssbusync][config] failed writing {}: {}",
                path,
                err
            );
            return Err(err);
        }
        Ok(())
    }

    fn ensure_default_profile(data: &mut ConfigFile) -> bool {
        if data.ssbusync.contains_key("Default") {
            false
        } else {
            data.ssbusync.insert("Default".to_string(), ProfileEntry::default());
            true
        }
    }

    pub fn load_or_create() -> io::Result<(SsbuSyncConfig, DefaultProfileState)> {
        paths::ensure_paths_exist()?;
        let path = paths::ssbusync_config();
        let mut data = load_file(&path)?;

        let changed = ensure_default_profile(&mut data);
        if changed {
            write_file(&path, &data)?;
        }

        let config = data
            .ssbusync
            .get("Default")
            .map(|entry| entry.config)
            .unwrap_or_else(SsbuSyncConfig::default);

        let state = if changed {
            DefaultProfileState::Created
        } else {
            DefaultProfileState::Loaded
        };

        Ok((config, state))
    }

    pub fn get_or_make_profile(
        name: &str,
        defaults: &SsbuSyncConfig,
        version: f32,
    ) -> io::Result<SsbuSyncConfig> {
        paths::ensure_paths_exist()?;
        let path = paths::ssbusync_config();
        let mut data = load_file(&path)?;

        let mut changed = ensure_default_profile(&mut data);
        let entry = data.ssbusync.entry(name.to_string());
        let mut should_write = false;

        match entry {
            std::collections::btree_map::Entry::Occupied(mut occ) => {
                if occ.get().profile_version < version {
                    should_write = true;
                    occ.insert(ProfileEntry {
                        profile_version: version,
                        config: *defaults,
                    });
                }
            }
            std::collections::btree_map::Entry::Vacant(vac) => {
                should_write = true;
                vac.insert(ProfileEntry {
                    profile_version: version,
                    config: *defaults,
                });
            }
        }

        if should_write || changed {
            write_file(&path, &data)?;
        }

        Ok(data
            .ssbusync
            .get(name)
            .map(|entry| entry.config)
            .unwrap_or_else(|| *defaults))
    }
}

pub mod disablers {
    use super::paths;
    use arcropolis_api::{hash40, is_mod_enabled};
    use serde::{Deserialize, Serialize};
    use skyline::nn::ro;
    use std::io;

    #[derive(Debug, Serialize, Deserialize, Default)]
    #[serde(default)]
    struct DisablersFile {
        #[serde(rename = "SsbuSync")]
        ssbusync: DisablersEntry,
    }

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(default)]
    struct DisablersEntry {
        mods: Vec<String>,
    }

    impl Default for DisablersEntry {
        fn default() -> Self {
            Self {
                mods: vec!["hdr".to_string(), "hdr-dev".to_string()],
            }
        }
    }

    fn io_err(err: impl std::fmt::Display) -> io::Error {
        io::Error::new(io::ErrorKind::InvalidData, err.to_string())
    }

    fn arcropolis_api_available() -> bool {
        let mut addr = 0usize;
        unsafe { ro::LookupSymbol(&mut addr, b"arcrop_api_version\0".as_ptr()) == 0 && addr != 0 }
    }

    fn load_file(path: &camino::Utf8PathBuf) -> io::Result<DisablersFile> {
        if !path.exists() {
            return Ok(DisablersFile::default());
        }
        let data = match std::fs::read_to_string(path.as_std_path()) {
            Ok(data) => data,
            Err(err) => {
                println!("[ssbusync][disablers] failed reading {}: {}", path, err);
                return Err(err);
            }
        };

        match toml::from_str(&data) {
            Ok(parsed) => Ok(parsed),
            Err(err) => {
                println!(
                    "[ssbusync][disablers] failed parsing TOML {}: {}",
                    path, err
                );
                Err(io_err(err))
            }
        }
    }

    fn write_file(path: &camino::Utf8PathBuf, data: &DisablersFile) -> io::Result<()> {
        let toml = match toml::to_string_pretty(data) {
            Ok(toml) => toml,
            Err(err) => {
                println!(
                    "[ssbusync][disablers] failed serializing TOML {}: {}",
                    path, err
                );
                return Err(io_err(err));
            }
        };

        if let Err(err) = std::fs::write(path.as_std_path(), toml) {
            println!("[ssbusync][disablers] failed writing {}: {}", path, err);
            return Err(err);
        }
        Ok(())
    }

    fn normalize_mod_path(entry: &str) -> Option<String> {
        let entry = entry.trim();
        if entry.is_empty() {
            return None;
        }
        if entry.starts_with("sd:/") {
            return Some(entry.to_string());
        }
        Some(format!(
            "sd:/ultimate/mods/{}",
            entry.trim_start_matches('/')
        ))
    }

    fn emulator_mod_path_candidates(entry: &str) -> Vec<String> {
        let mut out = Vec::new();
        if let Some(sd_path) = normalize_mod_path(entry) {
            out.push(sd_path.clone());
            if let Some(stripped) = sd_path.strip_prefix("sd:/") {
                out.push(stripped.to_string());
            }
        }

        let trimmed = entry.trim().trim_start_matches('/');
        if !trimmed.is_empty() && !trimmed.starts_with("sd:/") {
            out.push(format!("mods/{}", trimmed));
            out.push(format!("ultimate/mods/{}", trimmed));
        }

        out.sort();
        out.dedup();
        out
    }

    fn match_disabler_folder(mods: &[String]) -> Option<String> {
        for entry in mods {
            for candidate in emulator_mod_path_candidates(entry) {
                if std::path::Path::new(&candidate).is_dir() {
                    return Some(candidate);
                }
            }
        }
        None
    }

    pub fn load_or_create() -> io::Result<Vec<String>> {
        paths::ensure_paths_exist()?;
        let path = paths::ssbusync_disablers();
        let created = !path.exists();
        let data = load_file(&path)?;
        if created {
            write_file(&path, &data)?;
            println!(
                "[ssbusync][disablers] created {} with default disablers: hdr, hdr-dev",
                path
            );
        }
        Ok(data.ssbusync.mods)
    }

    pub fn active_disabler_mod() -> io::Result<Option<String>> {
        let mods = load_or_create()?;
        if mods.is_empty() {
            return Ok(None);
        }

        if crate::emulator_status() || !arcropolis_api_available() {
            if let Some(path) = match_disabler_folder(&mods) {
                println!(
                    "[ssbusync][disablers] folder check matched disabler '{}'",
                    path
                );
                return Ok(Some(path));
            }
            return Ok(None);
        }

        for entry in &mods {
            let Some(path) = normalize_mod_path(entry) else {
                continue;
            };
            if is_mod_enabled(hash40(path.as_str())) {
                return Ok(Some(path));
            }
        }
        Ok(None)
    }

    pub fn check_disabler_mods() -> bool {
        match active_disabler_mod() {
            Ok(Some(mod_path)) => {
                println!(
                    "[ssbusync] disablers.toml matched active mod '{}'; disabling ssbusync install",
                    mod_path
                );
                true
            }
            Ok(None) => false,
            Err(err) => {
                println!(
                    "[ssbusync] failed reading disablers.toml ({}); continuing install checks",
                    err
                );
                false
            }
        }
    }
}
