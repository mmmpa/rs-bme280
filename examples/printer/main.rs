use bme280::unix::Bme280Client;
use bme280::{Bme280, SetUpParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    serve().await
}

async fn serve() -> Result<(), Box<dyn std::error::Error>> {
    let mut bme = Bme280Client::new_with_path_and_address_hex(
        env!("I2C_DEVICE_PATH"),
        env!("I2C_DEVICE_ADDRESS"),
    )?;
    bme.reset()?;
    bme.set_up(SetUpParams::default())?;

    // The calibrator is reusable.
    let cal = bme.fetch_calibration()?;

    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        let (t, h, p) = bme.get_calibrated_results(&cal).unwrap();

        println!("temperature: {}, humidity: {}, pressure: {}", t, h, p);
    }
}
