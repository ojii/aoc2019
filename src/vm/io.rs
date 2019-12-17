use std::collections::VecDeque;
use std::sync::mpsc::{Receiver, Sender};

pub type IOResult<T> = Option<T>;

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

    fn read(&mut self) -> IOResult<i64> {
        self.input.read()
    }

    fn write(&mut self, value: i64) -> IOResult<()> {
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
        Some(*self)
    }
}

impl Output for i64 {
    type Value = i64;
    fn write(&mut self, value: i64) -> IOResult<()> {
        *self = value;
        Some(())
    }

    fn output(self) -> i64 {
        self
    }
}

impl Input for VecDeque<i64> {
    fn read(&mut self) -> IOResult<i64> {
        self.pop_front()
    }
}

impl Output for Vec<i64> {
    type Value = Vec<i64>;

    fn write(&mut self, value: i64) -> IOResult<()> {
        self.push(value);
        Some(())
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
        println!("Cannot read from null io");
        None
    }
}

impl Output for NullIO {
    type Value = ();
    fn write(&mut self, _value: i64) -> IOResult<()> {
        println!("Cannot write to null io");
        None
    }

    fn output(self) {}
}

impl Input for Receiver<i64> {
    fn read(&mut self) -> IOResult<i64> {
        self.recv().ok()
    }
}

impl Output for Sender<i64> {
    type Value = ();
    fn write(&mut self, value: i64) -> IOResult<()> {
        self.send(value).ok()
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

    fn write(&mut self, value: i64) -> IOResult<()> {
        if self.sender.send(value).is_err() {
            self.store.push(value);
        };
        Some(())
    }

    fn output(self) -> Vec<i64> {
        self.store
    }
}
