use std::collections::VecDeque;
use std::sync::mpsc::{Receiver, Sender};

#[allow(dead_code)]
pub type IOResult<T> = Result<T, String>;

pub trait Input {
    fn read(&mut self) -> IOResult<i64>;
}

pub trait Output {
    fn write(&mut self, value: i64) -> IOResult<()>;
}

impl Input for i64 {
    fn read(&mut self) -> IOResult<i64> {
        Ok(*self)
    }
}

impl Output for i64 {
    fn write(&mut self, value: i64) -> IOResult<()> {
        *self = value;
        Ok(())
    }
}

impl Input for VecDeque<i64> {
    fn read(&mut self) -> IOResult<i64> {
        self.pop_front()
            .map(Ok)
            .unwrap_or_else(|| Err("Input empty".to_string()))
    }
}

impl Output for Vec<i64> {
    fn write(&mut self, value: i64) -> IOResult<()> {
        self.push(value);
        Ok(())
    }
}

#[derive(Default)]
pub struct NullIO {}

impl NullIO {
    pub fn new() -> NullIO {
        NullIO {}
    }
}

impl Input for NullIO {
    fn read(&mut self) -> IOResult<i64> {
        Err("Cannot read from null io".to_string())
    }
}

impl Output for NullIO {
    fn write(&mut self, _value: i64) -> IOResult<()> {
        Err("Cannot write to null io".to_string())
    }
}

impl Input for Receiver<i64> {
    fn read(&mut self) -> IOResult<i64> {
        self.recv().map(Ok).unwrap_or_else(|e| Err(e.to_string()))
    }
}

impl Output for Sender<i64> {
    fn write(&mut self, value: i64) -> IOResult<()> {
        self.send(value).unwrap();
        Ok(())
    }
}

pub struct SendOrStore {
    sender: Sender<i64>,
    pub store: Vec<i64>,
}

impl SendOrStore {
    pub fn new(sender: Sender<i64>) -> Self {
        Self {
            sender,
            store: Vec::new(),
        }
    }
}

impl Output for SendOrStore {
    fn write(&mut self, value: i64) -> Result<(), String> {
        match self.sender.send(value) {
            Err(_) => self.store.push(value),
            Ok(_) => (),
        };
        Ok(())
    }
}
