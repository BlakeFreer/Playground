use embassy_stm32::{peripherals::RTC, rtc::Rtc};

pub struct Ticker {
    rtc: Rtc,
}

// impl Ticker {
//     pub fn new(rtc0: embassy_stm32::Peri<'_, RTC>) -> Self {
//         let (rtc, _) = Rtc::new(rtc0, Default::default());
//         rtc.get_daylight_savings
//         Self { rtc }
//     }
//
//     pub fn now(&self) -> u32 {
//         self.rtc.get_counter();
//     }
// }
