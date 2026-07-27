#![no_std]
#![no_main]

mod time;

use defmt::info;
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_stm32::adc::Temperature;
use embassy_stm32::exti::{self, ExtiInput};
use embassy_stm32::gpio::{AnyPin, Input, Level, Output, Pin, Pull};
use embassy_stm32::{Peri, bind_interrupts, interrupt};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::signal::Signal;
use embassy_time::{Duration, Timer, WithTimeout};
use panic_probe as _;

bind_interrupts!(
    pub struct Irqs{
        EXTI4_15 => exti::InterruptHandler<interrupt::typelevel::EXTI4_15>;
    }
);

// which button was pressed
#[derive(Clone, Copy)]
enum Button {
    A,
    B,
}

static SIGNAL: Signal<ThreadModeRawMutex, Button> = Signal::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Starting...");

    let p = embassy_stm32::init(Default::default());

    let b1 = ExtiInput::new(p.PC13, p.EXTI13, Pull::Up, Irqs);
    let b2 = ExtiInput::new(p.PA9, p.EXTI9, Pull::Up, Irqs);

    spawner
        .spawn(led_task(p.PA5.into(), Duration::from_millis(700)))
        .unwrap();

    spawner.spawn(temp_task()).unwrap();

    let b1_fut = button_task(b1, "B1", Button::A);
    let b2_fut = button_task(b2, "B2", Button::B);

    join(b1_fut, b2_fut).await;
}

async fn button_task(mut button: ExtiInput<'static>, id: &str, b: Button) {
    let mut count: u8 = 0;
    loop {
        button.wait_for_low().await;
        count += 1;
        info!("Pressed {} ({}) (fut)", id, count);
        SIGNAL.signal(b);
        button.wait_for_high().await;
        info!("Released {}", id);
    }
}

#[embassy_executor::task]
async fn led_task(led: Peri<'static, AnyPin>, duration: Duration) {
    let mut led = Output::new(led, Level::Low, embassy_stm32::gpio::Speed::Low);

    loop {
        led.set_high();
        info!("LED On");
        Timer::after(duration).await;
        led.set_low();
        info!("LED Off");
        Timer::after(duration).await;
    }
}

#[embassy_executor::task]
async fn temp_task() {
    const INTERVAL_MS: u64 = 500;
    let mut delay_ms = 500;
    loop {
        let value = 23;
        info!("{} C", value);
        let delay = Duration::from_millis(delay_ms);

        // Use the A/B buttons to change the data rate
        // When pressed, each button sets the SIGNAL. This task picks up on it
        if let Ok(v) = SIGNAL.wait().with_timeout(delay).await {
            delay_ms = match v {
                Button::A if delay_ms > INTERVAL_MS => delay_ms - INTERVAL_MS,
                Button::A => delay_ms,
                Button::B => delay_ms + INTERVAL_MS,
            };
            info!("Delay = {} ms", delay_ms);
        }
    }
}
