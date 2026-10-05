use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
    io::stdout,
};

use color_eyre::Result;
use crossterm::event::MouseEventKind;
use ratatui::crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};
use ratatui::crossterm::{execute, terminal::SetSize};

use crate::common::trace_log;

#[derive(Clone, Copy, Debug)]
pub enum Event {
    Tick,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

#[derive(Debug)]
pub struct EventHandler {
    #[allow(dead_code)]
    sender: mpsc::SyncSender<Event>,
    receiver: mpsc::Receiver<Event>,
    #[allow(dead_code)]
    handler: thread::JoinHandle<()>,
}

impl EventHandler {
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::sync_channel(10);
        let handler = {
            let sender = sender.clone();
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);

                    if event::poll(timeout).expect("unable to poll for event") {
                        match event::read().expect("unable to read event") {
                            CrosstermEvent::Key(e) => {
                                if e.kind == event::KeyEventKind::Press {
                                    let _ = sender.send(Event::Key(e));
                                }
                            }
                            CrosstermEvent::Mouse(e) => {
                                if let Ok(mut file) = std::fs::OpenOptions::new().append(true).create(true).open("mouse_raw.log") {
                                    use std::io::Write;
                                    let _ = writeln!(file, "RAW EVENT: {:?}", e.kind);
                                }
                                let _ = sender.send(Event::Mouse(e));
                            },
                            CrosstermEvent::Resize(w, h) => {
                                let _ = sender.send(Event::Resize(w, h));
                            },
                            _ => {},
                        }
                    }

                    if last_tick.elapsed() >= tick_rate {
                        sender.send(Event::Tick)
                            .map_err(|e| trace_log(&format!("failed to send tick event: {:?}", e)))
                            .expect("channel error");
                        last_tick = Instant::now();
                    }
                }
            })
        };
        Self {
            sender,
            receiver,
            handler,
        }
    }

    pub fn next(&self) -> Result<Event> {
        Ok(self.receiver.recv()?)
    }

    pub fn try_next(&self) -> Result<Option<Event>> {
        match self.receiver.try_recv() {
            Ok(event) => Ok(Some(event)),
            Err(mpsc::TryRecvError::Empty) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}