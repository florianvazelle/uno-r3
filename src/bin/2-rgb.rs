#![no_std]
#![no_main]

extern crate panic_halt;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut red = pins.d6.into_output();
    let mut green = pins.d5.into_output();
    let mut blue = pins.d3.into_output();

    loop {
        (0..3).for_each(|i| {
            red.set_low();
            green.set_low();
            blue.set_low();

            match i {
                0 => red.set_high(),
                1 => green.set_high(),
                2 => blue.set_high(),
                _ => (),
            }
            arduino_hal::delay_ms(200);
        });
    }
}