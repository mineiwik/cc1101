/// Sets the TX power.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum PowerLevel {
    /// 10 dBm (10 mW)
    Power10Dbm = 0xC5,
    /// 5 dBm (3.16 mW)
    Power5Dbm = 0x86,
    /// 0 dBm (1 mW)
    Power0Dbm = 0x50,
}

impl From<PowerLevel> for u8 {
    fn from(value: PowerLevel) -> Self {
        value as Self
    }
}
