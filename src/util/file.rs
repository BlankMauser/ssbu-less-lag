pub mod paths {
    use camino::Utf8PathBuf;
    use std::io;

    pub fn ensure_paths_exist() -> io::Result<()> {
        std::fs::create_dir_all(ssbusync())?;
        Ok(())
    }

    pub fn ssbusync() -> Utf8PathBuf {
        Utf8PathBuf::from("sd:/ultimate/ssbusync")
    }

    pub fn ssbusync_config() -> Utf8PathBuf {
        ssbusync().join("ssbusync.toml")
    }
}

pub mod config {
    use super::paths;
    use crate::SsbuSyncConfig;
    use serde::{Deserialize, Serialize};
    use std::collections::BTreeMap;
    use std::io;

    const DEFAULT_PROFILE_VERSION: f32 = 1.0;

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
        let data = std::fs::read_to_string(path.as_std_path())?;
        toml::from_str(&data).map_err(io_err)
    }

    fn write_file(path: &camino::Utf8PathBuf, data: &ConfigFile) -> io::Result<()> {
        let toml = toml::to_string_pretty(data).map_err(io_err)?;
        std::fs::write(path.as_std_path(), toml)?;
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

    pub fn load_or_create() -> io::Result<SsbuSyncConfig> {
        paths::ensure_paths_exist()?;
        let path = paths::ssbusync_config();
        let mut data = load_file(&path)?;

        let changed = ensure_default_profile(&mut data);
        if changed {
            write_file(&path, &data)?;
        }

        Ok(data
            .ssbusync
            .get("Default")
            .map(|entry| entry.config)
            .unwrap_or_else(SsbuSyncConfig::default))
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
