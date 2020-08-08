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
    pub t1: u16,
    pub t2: i16,
    pub t3: i16,

    pub p1: u16,
    pub p2: i16,
    pub p3: i16,
    pub p4: i16,
    pub p5: i16,
    pub p6: i16,
    pub p7: i16,
    pub p8: i16,
    pub p9: i16,

    pub h1: u8,
    pub h2: i16,
    pub h3: u8,
    pub h4: i16,
    pub h5: i16,
    pub h6: i8,
}

pub trait Bme280 {
    type I2c: I2c;

    fn i2c(&self) -> &Self::I2c;

    fn set_calibration(&mut self) -> Bme280Result<Calibrator> {
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
            t1,
            t2,
            t3,
            p1,
            p2,
            p3,
            p4,
            p5,
            p6,
            p7,
            p8,
            p9,
            h1,
            h2,
            h3,
            h4,
            h5,
            h6,
        })
    }
}
