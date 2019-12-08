use std::collections::VecDeque;

#[allow(dead_code)]

pub enum IOResult<T> {
    Ok(T),
    Error(String),
}

pub trait Input: Clone {
    fn read(&mut self) -> IOResult<i64>;
}

pub trait Output: Clone {
    fn write(&mut self, value: i64) -> IOResult<()>;
}

impl Input for i64 {
    fn read(&mut self) -> IOResult<i64> {
        IOResult::Ok(*self)
    }
}

impl Output for i64 {
    fn write(&mut self, value: i64) -> IOResult<()> {
        *self = value;
        IOResult::Ok(())
    }
}

impl Input for VecDeque<i64> {
    fn read(&mut self) -> IOResult<i64> {
        self.pop_front()
            .map(|value| IOResult::Ok(value))
            .unwrap_or_else(|| IOResult::Error("Input empty".to_string()))
    }
}

impl Output for Vec<i64> {
    fn write(&mut self, value: i64) -> IOResult<()> {
        self.push(value);
        IOResult::Ok(())
    }
}

#[derive(Clone, Default)]
pub struct NullIO {}

impl NullIO {
    pub fn new() -> NullIO {
        NullIO {}
    }
}

impl Input for NullIO {
    fn read(&mut self) -> IOResult<i64> {
        IOResult::Error("Cannot read from null io".to_string())
    }
}

impl Output for NullIO {
    fn write(&mut self, _value: i64) -> IOResult<()> {
        IOResult::Error("Cannot write to null io".to_string())
    }
}
