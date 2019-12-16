use std::collections::VecDeque;
use std::sync::mpsc::{Receiver, Sender};

pub type IOResult<T> = Result<T, String>;

pub trait IO {
    type Value;
    fn read(&mut self) -> IOResult<i64>;
    fn write(&mut self, value: i64) -> IOResult<()>;
    fn output(self) -> Self::Value;
}

pub struct InputOutput<I: Input, O: Output> {
    input: I,
    output: O,
}

impl<I: Input, O: Output> InputOutput<I, O> {
    pub fn new(input: I, output: O) -> Self {
        Self { input, output }
    }
}

impl<I: Input, O: Output> IO for InputOutput<I, O> {
    type Value = O::Value;

    fn read(&mut self) -> Result<i64, String> {
        self.input.read()
    }

    fn write(&mut self, value: i64) -> Result<(), String> {
        self.output.write(value)
    }

    fn output(self) -> O::Value {
        self.output.output()
    }
}

pub trait Input {
    fn read(&mut self) -> IOResult<i64>;
}

pub trait Output {
    type Value;
    fn write(&mut self, value: i64) -> IOResult<()>;
    fn output(self) -> Self::Value;
}

impl Input for i64 {
    fn read(&mut self) -> IOResult<i64> {
        Ok(*self)
    }
}

impl Output for i64 {
    type Value = i64;
    fn write(&mut self, value: i64) -> IOResult<()> {
        *self = value;
        Ok(())
    }

    fn output(self) -> i64 {
        self
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
    type Value = Vec<i64>;

    fn write(&mut self, value: i64) -> IOResult<()> {
        self.push(value);
        Ok(())
    }

    fn output(self) -> Vec<i64> {
        self
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
    type Value = ();
    fn write(&mut self, _value: i64) -> IOResult<()> {
        Err("Cannot write to null io".to_string())
    }

    fn output(self) {}
}

impl Input for Receiver<i64> {
    fn read(&mut self) -> IOResult<i64> {
        self.recv().map(Ok).unwrap_or_else(|e| Err(e.to_string()))
    }
}

impl Output for Sender<i64> {
    type Value = ();
    fn write(&mut self, value: i64) -> IOResult<()> {
        self.send(value).unwrap();
        Ok(())
    }

    fn output(self) {}
}

pub struct SendOrStore {
    sender: Sender<i64>,
    store: Vec<i64>,
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
    type Value = Vec<i64>;

    fn write(&mut self, value: i64) -> Result<(), String> {
        if self.sender.send(value).is_err() {
            self.store.push(value);
        };
        Ok(())
    }

    fn output(self) -> Vec<i64> {
        self.store
    }
}
