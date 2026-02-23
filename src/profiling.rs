use std::{
    collections::BTreeMap,
    io::Write,
    net::{Ipv4Addr, SocketAddrV4, TcpListener},
    sync::{
        mpsc::{Receiver, Sender},
        OnceLock,
    },
};

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct OsTick(pub i64);

impl OsTick {
    pub const fn new(tick: i64) -> Self {
        Self(tick)
    }

    pub const fn from_nanos(nanos: i64) -> Self {
        Self((nanos * 12) / 625)
    }

    pub const fn to_nanos(self) -> i64 {
        (self.0 * 625) / 12
    }
}

pub struct Span {
    pub name: &'static str,
    pub start: OsTick,
    pub end: OsTick,
}

enum Item {
    /// Indicates that we have begun processing a new game frame
    FrameStart { tick: OsTick },
    /// Submits the current processing frame with a frame index, this allows us to begin
    /// recording a new frame
    SubmitFrame { frame: usize, tick: OsTick },
    /// Starts a span on the current frame with the given name
    StartSpan { name: &'static str, tick: OsTick },
    /// Ends the current span being processed
    EndSpan { tick: OsTick },
    /// Indicates that a frame has been presented
    FinishFrame { frame: usize, tick: OsTick },
    /// Records the timestamp of a vblank
    VBlank { tick: OsTick },
}

static SENDER: OnceLock<Sender<Item>> = OnceLock::new();

#[inline]
fn sender() -> Option<&'static Sender<Item>> {
    SENDER.get()
}

pub fn start_frame(tick: OsTick) {
    if let Some(sender) = sender() {
        let _ = sender.send(Item::FrameStart { tick });
    }
}

pub fn submit_frame(frame: usize, tick: OsTick) {
    if let Some(sender) = sender() {
        let _ = sender.send(Item::SubmitFrame { frame, tick });
    }
}

pub fn finish_frame(frame: usize, tick: OsTick) {
    if let Some(sender) = sender() {
        let _ = sender.send(Item::FinishFrame { frame, tick });
    }
}

pub fn start_span(name: &'static str, tick: OsTick) {
    if let Some(sender) = sender() {
        let _ = sender.send(Item::StartSpan { name, tick });
    }
}

pub fn end_span(tick: OsTick) {
    if let Some(sender) = sender() {
        let _ = sender.send(Item::EndSpan { tick });
    }
}

pub fn span(name: &'static str, start: OsTick, end: OsTick) {
    start_span(name, start);
    end_span(end);
}

pub fn vblank(tick: OsTick) {
    if let Some(sender) = sender() {
        let _ = sender.send(Item::VBlank { tick });
    }
}

struct FrameSpan {
    name: &'static str,
    start: OsTick,
    end: OsTick,
}

struct SubmittedFrame {
    start: OsTick,
    submitted_at: OsTick,
    spans: Vec<FrameSpan>,
}

fn serialize_span(span: &FrameSpan, buffer: &mut Vec<u8>) {
    buffer.extend_from_slice(&span.start.0.to_le_bytes());
    buffer.extend_from_slice(&span.end.0.to_le_bytes());
    buffer.extend_from_slice(&span.name.len().to_le_bytes());
    buffer.extend_from_slice(span.name.as_bytes());
}

fn serialize_frame(
    frame: &SubmittedFrame,
    frame_index: u64,
    dangled: bool,
    presented: OsTick,
    buffer: &mut Vec<u8>,
) {
    buffer.push(0u8);
    buffer.extend_from_slice(&frame_index.to_le_bytes());
    buffer.extend_from_slice(&frame.start.0.to_le_bytes());
    buffer.extend_from_slice(&frame.submitted_at.0.to_le_bytes());
    buffer.extend_from_slice(&presented.0.to_le_bytes());
    buffer.extend_from_slice(&[dangled as u8]);
    buffer.extend_from_slice(&frame.spans.len().to_le_bytes());
    for span in frame.spans.iter() {
        serialize_span(span, buffer);
    }
}

fn thread(receiver: Receiver<Item>) {
    struct FrameInFlight {
        start: OsTick,
        completed_spans: Vec<FrameSpan>,
        current_span: Option<FrameSpan>,
    }

    let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 1776)).unwrap();

    let mut frame_in_flight = None;
    let mut submitted_frames: BTreeMap<usize, SubmittedFrame> = BTreeMap::new();

    'outer: loop {
        let (mut next, _) = listener.accept().unwrap();

        let mut buffer = vec![];

        loop {
            match receiver.recv() {
                Ok(item) => {
                    buffer.clear();
                    match item {
                        Item::FrameStart { tick } => {
                            if frame_in_flight.is_some() {
                                println!("[WARN] Profiler is skipping in-flight frame because another was started before it was submitted");
                            }

                            frame_in_flight = Some(FrameInFlight {
                                start: tick,
                                completed_spans: vec![],
                                current_span: None,
                            });
                        }
                        Item::SubmitFrame {
                            frame: frame_index,
                            tick,
                        } => {
                            let Some(mut frame_in_flight) = frame_in_flight.take() else {
                                println!("[WARN] Profile has submitted frame but no in-flight frame found");
                                continue;
                            };

                            if let Some(span) = frame_in_flight.current_span.take() {
                                frame_in_flight.completed_spans.push(span);
                            }

                            submitted_frames.insert(
                                frame_index,
                                SubmittedFrame {
                                    start: frame_in_flight.start,
                                    spans: frame_in_flight.completed_spans,
                                    submitted_at: tick,
                                },
                            );
                        }
                        Item::FinishFrame {
                            frame: frame_index,
                            tick,
                        } => {
                            let Some(frame) = submitted_frames.remove(&frame_index) else {
                                println!("[WARN] Finished frame but it is not in the submitted frames list");
                                continue;
                            };

                            while submitted_frames
                                .first_entry()
                                .is_some_and(|frame| *frame.key() < frame_index)
                            {
                                let (frame_index, dangled) =
                                    unsafe { submitted_frames.pop_first().unwrap_unchecked() };
                                println!("[ERROR] Dangling frame {frame_index} left in profiler");
                                serialize_frame(
                                    &dangled,
                                    frame_index as u64,
                                    true,
                                    tick,
                                    &mut buffer,
                                );
                            }

                            serialize_frame(&frame, frame_index as u64, false, tick, &mut buffer);
                        }
                        Item::StartSpan { name, tick } => {
                            let Some(frame) = frame_in_flight.as_mut() else {
                                println!(
                                    "[WARN] Cannot start span for frame since one is not in flight"
                                );
                                continue;
                            };

                            if let Some(mut old) = frame.current_span.take() {
                                if old.name == name {
                                    frame.current_span = Some(old);
                                    continue;
                                } else {
                                    old.end = tick;
                                    frame.completed_spans.push(old);
                                }
                            }

                            frame.current_span = Some(FrameSpan {
                                name,
                                start: tick,
                                end: tick,
                            });
                        }
                        Item::EndSpan { tick } => {
                            let Some(frame) = frame_in_flight.as_mut() else {
                                println!(
                                    "[WARN] Cannot end span for frame since one is not in flight"
                                );
                                continue;
                            };

                            if let Some(mut old) = frame.current_span.take() {
                                old.end = tick;
                                frame.completed_spans.push(old);
                            }
                        }
                        Item::VBlank { tick } => {
                            buffer.push(1u8);
                            buffer.extend_from_slice(&tick.0.to_le_bytes());
                        }
                    }

                    if next.write_all(&buffer).is_err() {
                        break;
                    }
                }
                Err(_) => break 'outer,
            }
        }
    }
}

pub fn setup() {
    let (sender, receiver) = std::sync::mpsc::channel();

    SENDER
        .set(sender)
        .expect("only call profiling::setup() once");

    std::thread::spawn(move || thread(receiver));
}
