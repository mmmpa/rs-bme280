#![cfg_attr(not(feature = "std"), no_std)]

mod bme280;
mod client;

pub use crate::bme280::*;

pub type Bme280Result<T> = Result<T, Bme280Error>;
