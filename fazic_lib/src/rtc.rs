pub trait Rtc {
    fn now(&mut self) -> u128;
    fn elapsed(&mut self) -> u128;
}

pub struct DummyRtc {}

impl DummyRtc {
    pub fn new() -> DummyRtc {
        DummyRtc {}
    }
}

impl Rtc for DummyRtc {
    fn now(&mut self) -> u128 {
        0
    }
    fn elapsed(&mut self) -> u128 {
        0
    }
}
