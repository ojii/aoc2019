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
            .map(|value| Ok(value))
            .unwrap_or_else(|| Err("Input empty".to_string()))
    }
}

impl Output for Vec<i64> {
    fn write(&mut self, value: i64) -> IOResult<()> {
        self.push(value);
        Ok(())
    }
}

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
        self.recv()
            .map(|value| Ok(value))
            .unwrap_or_else(|e| Err(e.to_string()))
    }
}

impl Output for Sender<i64> {
    fn write(&mut self, value: i64) -> IOResult<()> {
        self.send(value).unwrap();
        Ok(())
    }
}

struct PeekingOutput<O: Output> {
    inner: O,
    last: Option<i64>,
}

impl<O: Output> Output for PeekingOutput<O> {
    fn write(&mut self, value: i64) -> Result<(), String> {
        self.last = Some(value);
        self.inner.write(value)
    }
}
