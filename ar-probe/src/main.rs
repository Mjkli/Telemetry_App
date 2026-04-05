
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::i2c::{I2cConfig, I2cDriver};
use esp_idf_hal::units::Hertz;



use mpu6050::Mpu6050;




fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();

    let mpu_sda = peripherals.pins.gpio11;
    let mpu_scl = peripherals.pins.gpio12;

    let config = I2cConfig::new().baudrate(Hertz(400_000));
    let i2c = I2cDriver::new(peripherals.i2c0, mpu_sda, mpu_scl, &config).unwrap();
    let mut mpu = Mpu6050::new(i2c);
    
    mpu.init(&mut esp_idf_hal::delay::FreeRtos).unwrap();




    loop {
        let accel = mpu.get_acc().unwrap();
        let gyro = mpu.get_gyro().unwrap();
        log::info!("Accel: x={:.2} y={:.2} z={:.2}", accel.x, accel.y, accel.z);
        log::info!("Gyro:  x={:.2} y={:.2} z={:.2}", gyro.x, gyro.y, gyro.z);
        FreeRtos::delay_ms(100);


    }
}
