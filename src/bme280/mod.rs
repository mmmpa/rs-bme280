mod error;

pub use error::*;

use crate::*;

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum RegisterAddress {
    Id = 0xD0,
    Reset = 0xE0,
    CtrlHum = 0xF2,
    Status = 0xF3,
    CtrlMeas = 0xF4,
    Config = 0xF5,

    PressM = 0xF7,
    PressL = 0xF8,
    PressXl = 0xF9,

    TempM = 0xFA,
    TempL = 0xFB,
    TempXl = 0xFC,

    HumM = 0xFD,
    HumL = 0xFE,

    T1L = 0x88,
    T1H = 0x89,
    T2L = 0x8A,
    T2H = 0x8B,
    T3L = 0x8C,
    T3H = 0x8D,

    P1L = 0x8E,
    P1H = 0x8F,
    P2L = 0x90,
    P2H = 0x91,
    P3L = 0x92,
    P3H = 0x93,
    P4L = 0x94,
    P4H = 0x95,
    P5L = 0x96,
    P5H = 0x97,
    P6L = 0x98,
    P6H = 0x99,
    P7L = 0x9A,
    P7H = 0x9B,
    P8L = 0x9C,
    P8H = 0x9D,
    P9L = 0x9E,
    P9H = 0x9F,

    H1 = 0xA1,
    H2L = 0xE1,
    H2H = 0xE2,
    H3 = 0xE3,
    H4H = 0xE4,
    H4LH5L = 0xE5,
    H5H = 0xE6,
    H6 = 0xE7,
}

const CALIBRATIONS: [RegisterAddress; 32] = [
    RegisterAddress::T1L,
    RegisterAddress::T1H,
    RegisterAddress::T2L,
    RegisterAddress::T2H,
    RegisterAddress::T3L,
    RegisterAddress::T3H,
    RegisterAddress::P1L,
    RegisterAddress::P1H,
    RegisterAddress::P2L,
    RegisterAddress::P2H,
    RegisterAddress::P3L,
    RegisterAddress::P3H,
    RegisterAddress::P4L,
    RegisterAddress::P4H,
    RegisterAddress::P5L,
    RegisterAddress::P5H,
    RegisterAddress::P6L,
    RegisterAddress::P6H,
    RegisterAddress::P7L,
    RegisterAddress::P7H,
    RegisterAddress::P8L,
    RegisterAddress::P8H,
    RegisterAddress::P9L,
    RegisterAddress::P9H,
    RegisterAddress::H1,
    RegisterAddress::H2L,
    RegisterAddress::H2H,
    RegisterAddress::H3,
    RegisterAddress::H4H,
    RegisterAddress::H4LH5L,
    RegisterAddress::H5H,
    RegisterAddress::H6,
];

pub trait I2c {
    fn write_i2c_block_data(&self, reg: RegisterAddress, data: &[u8]) -> Bme280Result<()>;
    fn write_byte_data(&self, reg: RegisterAddress, data: u8) -> Bme280Result<()>;
    fn read_byte_data(&self, reg: RegisterAddress) -> Bme280Result<u8>;
    fn read_i2c_block_data(&self, reg: RegisterAddress, data: &mut [u8]) -> Bme280Result<()>;
}

pub struct Calibrator {
    pub t1: f64, // u16,
    pub t2: f64, // i16,
    pub t3: f64, // i16,

    pub p1: f64, // u16,
    pub p2: f64, // i16,
    pub p3: f64, // i16,
    pub p4: f64, // i16,
    pub p5: f64, // i16,
    pub p6: f64, // i16,
    pub p7: f64, // i16,
    pub p8: f64, // i16,
    pub p9: f64, // i16,

    pub h1: f64, // u8,
    pub h2: f64, // i16,
    pub h3: f64, // u8,
    pub h4: f64, // i16,
    pub h5: f64, // i16,
    pub h6: f64, // i8,
}

pub trait Bme280 {
    type I2c: I2c;

    fn i2c(&self) -> &Self::I2c;

    fn fetch_calibration(&self) -> Bme280Result<Calibrator> {
        let mut bytes = [0u8; 32];
        for (i, reg) in CALIBRATIONS.iter().enumerate() {
            let v = self.i2c().read_byte_data(*reg)?;
            bytes[i] = v;
        }

        let t1 = u16::from_be_bytes([bytes[1], bytes[0]]);
        let t2 = i16::from_be_bytes([bytes[3], bytes[2]]);
        let t3 = i16::from_be_bytes([bytes[5], bytes[4]]);

        let p1 = u16::from_be_bytes([bytes[7], bytes[6]]);
        let p2 = i16::from_be_bytes([bytes[9], bytes[8]]);
        let p3 = i16::from_be_bytes([bytes[11], bytes[10]]);
        let p4 = i16::from_be_bytes([bytes[13], bytes[12]]);
        let p5 = i16::from_be_bytes([bytes[15], bytes[14]]);
        let p6 = i16::from_be_bytes([bytes[17], bytes[16]]);
        let p7 = i16::from_be_bytes([bytes[19], bytes[18]]);
        let p8 = i16::from_be_bytes([bytes[21], bytes[20]]);
        let p9 = i16::from_be_bytes([bytes[23], bytes[22]]);

        let h1 = bytes[24];
        let h2 = i16::from_be_bytes([bytes[26], bytes[25]]);
        let h3 = bytes[27];

        let h4b = ((bytes[28] as u16) << 4) | (0x0F & bytes[29]) as u16;
        let h4 = i16::from_be_bytes([((h4b & 0xff00) >> 8) as u8, (h4b & 0xff) as u8]);

        let h5b = ((bytes[30] as u16) << 4) | ((bytes[29] >> 4) & 0x0F) as u16;
        let h5 = i16::from_be_bytes([((h5b & 0xff00) >> 8) as u8, (h5b & 0xff) as u8]);

        let h6 = i8::from_be_bytes([bytes[31]]);

        Ok(Calibrator {
            t1: t1 as f64,
            t2: t2 as f64,
            t3: t3 as f64,
            p1: p1 as f64,
            p2: p2 as f64,
            p3: p3 as f64,
            p4: p4 as f64,
            p5: p5 as f64,
            p6: p6 as f64,
            p7: p7 as f64,
            p8: p8 as f64,
            p9: p9 as f64,
            h1: h1 as f64,
            h2: h2 as f64,
            h3: h3 as f64,
            h4: h4 as f64,
            h5: h5 as f64,
            h6: h6 as f64,
        })
    }

    fn get_results(&self) -> Bme280Result<(AdcTemperature, AdcHumidity, AdcPressure)> {
        let adc_t = self.get_adc_temperature()?;
        let adc_h = self.get_adc_humidity()?;
        let adc_p = self.get_adc_pressure()?;

        Ok((adc_t, adc_h, adc_p))
    }

    fn get_calibrated_results(
        &self,
        calibrator: &Calibrator,
    ) -> Bme280Result<(Temperature, Humidity, Pressure)> {
        let (adc_t, adc_h, adc_p) = self.get_results()?;

        let (t, fine_t) = calibrate_temperature(calibrator, adc_t);
        let h = calibrate_humidity(calibrator, adc_h, fine_t);
        let p = calibrate_pressure(calibrator, adc_p, fine_t);

        Ok((t, h, p))
    }

    fn get_adc_humidity(&self) -> Bme280Result<AdcHumidity> {
        let l = self.i2c().read_byte_data(RegisterAddress::HumL)? as u16;
        let m = self.i2c().read_byte_data(RegisterAddress::HumM)? as u16;

        Ok(AdcHumidity((l << 8) | m))
    }

    fn get_adc_pressure(&self) -> Bme280Result<AdcPressure> {
        let l = self.i2c().read_byte_data(RegisterAddress::PressL)? as u32;
        let m = self.i2c().read_byte_data(RegisterAddress::PressM)? as u32;
        let xl = self.i2c().read_byte_data(RegisterAddress::PressXl)? as u32;

        Ok(AdcPressure((l << 12) | (m << 4) | (xl >> 4)))
    }

    fn get_adc_temperature(&self) -> Bme280Result<AdcTemperature> {
        let l = self.i2c().read_byte_data(RegisterAddress::TempL)? as u32;
        let m = self.i2c().read_byte_data(RegisterAddress::TempM)? as u32;
        let xl = self.i2c().read_byte_data(RegisterAddress::TempXl)? as u32;

        Ok(AdcTemperature((l << 12) | (m << 4) | (xl >> 4)))
    }
}
fn calibrate_humidity(
    calibrator: &Calibrator,
    adc_h: AdcHumidity,
    t_fine: FineTemperature,
) -> Humidity {
    let Calibrator {
        h1,
        h2,
        h3,
        h4,
        h5,
        h6,
        ..
    } = calibrator;

    let adc_h = *adc_h.as_ref() as f64;

    let mut var_h = t_fine.as_ref() - 76800.0;
    var_h = (adc_h - (h4 * 64.0 + h5 / 16384.0 * var_h))
        * (h2 / 65536.0 * (1.0 + h6 / 67108864.0 * var_h * (1.0 + h3 / 67108864.0 * var_h)));
    var_h = var_h * (1.0 - h1 * var_h / 524288.0);
    if var_h > 100.0 {
        var_h = 100.0;
    } else if var_h < 0.0 {
        var_h = 0.0;
    }

    Humidity(var_h)
}

fn calibrate_pressure(
    calibrator: &Calibrator,
    adc_p: AdcPressure,
    t_fine: FineTemperature,
) -> Pressure {
    let Calibrator {
        p1,
        p2,
        p3,
        p4,
        p5,
        p6,
        p7,
        p8,
        p9,
        ..
    } = calibrator;

    let adc_p = *adc_p.as_ref() as f64;

    let mut var1 = t_fine.as_ref() / 2.0 - 64000.0;
    let mut var2 = var1 * var1 * (p6) / 32768.0;
    var2 = var2 + var1 * (p5) * 2.0;
    var2 = (var2 / 4.0) + ((p4) * 65536.0);
    var1 = ((p3) * var1 * var1 / 524288.0 + (p2) * var1) / 524288.0;
    var1 = (1.0 + var1 / 32768.0) * (p1);
    if var1 == 0.0 {
        return Pressure(0.0); // avoid exception caused by division by zero
    }
    let mut p = 1048576.0 - adc_p;
    p = (p - (var2 / 4096.0)) * 6250.0 / var1;
    var1 = (p9) * p * p / 2147483648.0;
    var2 = p * (p8) / 32768.0;
    p = p + (var1 + var2 + (p7)) / 16.0;

    Pressure(p)
}

fn calibrate_temperature(
    calibrator: &Calibrator,
    adc_t: AdcTemperature,
) -> (Temperature, FineTemperature) {
    let Calibrator { t1, t2, t3, .. } = calibrator;

    let adc_t = *adc_t.as_ref() as f64;

    let var1 = (adc_t / 16384.0 - t1 / 1024.0) * t2;
    let var2 = ((adc_t / 131072.0 - t1 / 8192.0) * (adc_t / 131072.0 - t1 / 8192.0)) * t3;
    let t_fine = var1 + var2;
    let t = t_fine / 5120.0;

    (Temperature(t), FineTemperature(t_fine))
}

#[derive(Copy, Clone)]
pub struct AdcTemperature(u32);
#[derive(Copy, Clone)]
pub struct AdcPressure(u32);
#[derive(Copy, Clone)]
pub struct AdcHumidity(u16);

#[derive(Copy, Clone)]
pub struct Temperature(f64);
#[derive(Copy, Clone)]
pub struct FineTemperature(f64);
#[derive(Copy, Clone)]
pub struct Pressure(f64);
#[derive(Copy, Clone)]
pub struct Humidity(f64);

impl AsRef<u32> for AdcTemperature {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}

impl AsRef<u32> for AdcPressure {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}

impl AsRef<u16> for AdcHumidity {
    fn as_ref(&self) -> &u16 {
        &self.0
    }
}

impl AsRef<f64> for Temperature {
    fn as_ref(&self) -> &f64 {
        &self.0
    }
}

impl AsRef<f64> for FineTemperature {
    fn as_ref(&self) -> &f64 {
        &self.0
    }
}

impl AsRef<f64> for Pressure {
    fn as_ref(&self) -> &f64 {
        &self.0
    }
}

impl AsRef<f64> for Humidity {
    fn as_ref(&self) -> &f64 {
        &self.0
    }
}
