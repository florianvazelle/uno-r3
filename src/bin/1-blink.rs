#![no_std]
#![no_main]

extern crate panic_halt;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Digital pin 13 is also connected to an onboard LED marked "L"
    let mut led = pins.d13.into_output();
    led.set_high();

    loop {
        (0..25).map(|i| i * 10).for_each(|i| {
            led.toggle();
            arduino_hal::delay_ms(i as u16);
        });
    }
}