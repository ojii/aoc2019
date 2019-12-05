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

#[derive(Clone)]
pub struct StaticInput {
    value: i64,
}

impl StaticInput {
    pub fn new(value: i64) -> StaticInput {
        StaticInput { value }
    }
}

impl Input for StaticInput {
    fn read(&mut self) -> IOResult<i64> {
        IOResult::Ok(self.value)
    }
}

#[derive(Clone)]
pub struct VecOutput {
    data: Vec<i64>,
}

impl VecOutput {
    pub fn new() -> VecOutput {
        VecOutput { data: Vec::new() }
    }

    pub fn last(&self) -> Option<&i64> {
        self.data.last()
    }
}

impl Output for VecOutput {
    fn write(&mut self, value: i64) -> IOResult<()> {
        self.data.push(value);
        IOResult::Ok(())
    }
}

#[derive(Clone)]
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
    fn write(&mut self, value: i64) -> IOResult<()> {
        IOResult::Error("Cannot write to null io".to_string())
    }
}
