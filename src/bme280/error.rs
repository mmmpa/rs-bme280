#[derive(Debug)]
pub enum Bme280Error {
    #[cfg(feature = "std")]
    I2cError(String),
}

#[cfg(feature = "std")]
impl std::fmt::Display for Bme280Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Bme280Error {}
