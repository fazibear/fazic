use std::time::Instant;
use fazic::rtc::Rtc;

pub struct InstantRtc {
    now: Instant,
}

impl InstantRtc {
    pub fn new() -> InstantRtc {
        InstantRtc {
            now: Instant::now() 
        }
    }
}

impl Rtc for InstantRtc {
    fn now(&mut self) -> u128 {
        self.now = Instant::now();
        self.now.elapsed().as_millis()
    }

    fn elapsed(&mut self) -> u128 {
        self.now.elapsed().as_millis()
    }

}

