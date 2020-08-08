#![no_std]

mod bme280;

pub use crate::bme280::*;

pub type Bme280Result<T> = Result<T, Bme280Error>;
