# bme280

This is to use BME280 with Rust.

- [BME280 Datasheet](https://ae-bst.resource.bosch.com/media/_tech/media/datasheets/BST-BME280-DS002.pdf)

# example for raspi zero

## env

```sh
export I2C_DEVICE_PATH="/dev/i2c-1"
export I2C_DEVICE_ADDRESS="0x76"
```

## .cargo/config

```toml
[target.arm-unknown-linux-gnueabi]
linker = "arm-linux-gnueabi-gcc"
```

## build

```shell
cargo build --example printer --target arm-unknown-linux-gnueabi --release --features std
```
