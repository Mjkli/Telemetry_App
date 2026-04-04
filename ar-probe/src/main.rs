
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::delay::FreeRtos;

fn main() {
    esp_idf_svc::sys::link_patches();


    let peripherals = Peripherals::take().unwrap();
    let mut led = PinDriver::output(peripherals.pins.gpio48).unwrap();
    let mut gled = PinDriver::output(peripherals.pins.gpio46).unwrap();
    


    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");


    led.set_low().unwrap();
    gled.set_low().unwrap();

    loop {
        gled.set_low().unwrap();
        led.set_high().unwrap();
        FreeRtos::delay_ms(500);
        gled.set_high().unwrap();
        led.set_low().unwrap();
        FreeRtos::delay_ms(500);

    }
}
