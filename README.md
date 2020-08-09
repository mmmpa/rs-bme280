# bme280

This is to use BME280 with Rust.

- [BME280 Datasheet](https://ae-bst.resource.bosch.com/media/_tech/media/datasheets/BST-BME280-DS002.pdf)

# Usage

```sh
export I2C_DEVICE_NUMBER="1"
export BME280_I2C_DEVICE_ADDRESS="0x76"
```

```toml
bme280 = { git = "https://github.com/mmmpa/rs_bme280", features = ["std"] }
```

```rust
use bme280::unix::Bme280Client;
use bme280::{Bme280, Calibrator, SetUp};

fn bme280() {
    let i2c_cli = LinuxI2CDevice::new(
        format!("/dev/i2c-{}", env!("I2C_DEVICE_NUMBER")),
        bme280_device_address(),
    )
    .unwrap();

    let mut bme = Bme280Client::new(i2c_cli);
    bme.reset().unwrap();
    bme.set_up(SetUp::default()).unwrap();

    // The calibrator is reusable.
    let cal = bme.fetch_calibration().unwrap();

    let (t, h, p) = bme.get_calibrated_results(&cal).unwrap();

    println!("temperature:{}, humidity:{}, pressure{}", t, h, p)
}

fn bme280_device_address() -> u16 {
    let no_prefix = env!("BME280_I2C_DEVICE_ADDRESS").trim_start_matches("0x");
    u16::from_str_radix(no_prefix, 16).unwrap()
}
```