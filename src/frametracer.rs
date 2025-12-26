use std::{
    collections::BTreeMap,
    fs::File,
    io::Write,
    sync::{
        mpsc::{Receiver, Sender},
        OnceLock,
    },
};

pub type Tick = i64;

struct FrameElement {
    name: &'static str,
    frame_index: usize,
    tick: Tick,
}

#[derive(serde::Serialize)]
struct MarkerElement {
    name: &'static str,
    tick: Tick,
}

enum Element {
    Frame(FrameElement),
    Marker(MarkerElement),
    FinishFrame(usize),
}

static SUBMISSION_CHANNEL: OnceLock<Sender<Element>> = OnceLock::new();

pub fn initialize() {
    let (sender, receiver) = std::sync::mpsc::channel();
    SUBMISSION_CHANNEL
        .set(sender)
        .expect("Only call frametracer::initialize() once");

    let _ = std::thread::spawn(move || frametracer_thread(receiver));
}

pub fn marker(name: &'static str, tick: Tick) {
    if let Some(channel) = SUBMISSION_CHANNEL.get() {
        let _ = channel.send(Element::Marker(MarkerElement { name, tick }));
    }
}

pub fn frame(name: &'static str, frame: usize, tick: Tick) {
    if let Some(channel) = SUBMISSION_CHANNEL.get() {
        let _ = channel.send(Element::Frame(FrameElement {
            name,
            frame_index: frame,
            tick,
        }));
    }
}

pub fn finish_frame(frame: usize) {
    if let Some(channel) = SUBMISSION_CHANNEL.get() {
        let _ = channel.send(Element::FinishFrame(frame));
    }
}

fn frametracer_thread(receiver: Receiver<Element>) {
    #[derive(serde::Serialize)]
    struct NamedTick {
        name: &'static str,
        tick: Tick,
    }

    let mut frames_in_flight: BTreeMap<usize, Vec<NamedTick>> = BTreeMap::new();

    let mut file = File::create("sd:/frametrace.jsonl").unwrap();

    loop {
        match receiver.recv() {
            Ok(Element::Frame(frame)) => {
                frames_in_flight
                    .entry(frame.frame_index)
                    .or_default()
                    .push(NamedTick {
                        name: frame.name,
                        tick: frame.tick,
                    });
            }
            Ok(Element::Marker(marker)) => {
                if let Ok(mut line) = serde_json::to_string(&marker) {
                    line.push('\n');
                    let _ = file.write_all(line.as_bytes());
                    let _ = file.flush();
                }
            }
            Ok(Element::FinishFrame(frame)) => {
                #[derive(serde::Serialize)]
                struct FrameEntry {
                    frame: usize,
                    elements: Vec<NamedTick>,
                }

                let frame = FrameEntry {
                    frame,
                    elements: frames_in_flight.remove(&frame).unwrap_or_default(),
                };
                if let Ok(mut line) = serde_json::to_string(&frame) {
                    line.push('\n');
                    let _ = file.write_all(line.as_bytes());
                    let _ = file.flush();
                }
            }
            Err(_error) => break,
        }
    }
}
