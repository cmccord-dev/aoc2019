pub trait IntcodeIO {
    fn read(&self) -> i64;
    fn write(&mut self, data: i64);
}
