/// DVGA setting
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum DVGASetting {
    // All gain settings can be used
    All = 0,
    // The highest gain setting can not be used
    AllButHighest = 1,
    // The highest 2 gain settings can not be used
    AllButHighest2 = 2,
    // The highest 3 gain settings can not be used
    AllButHighest3 = 3,
}

impl From<DVGASetting> for u8 {
    fn from(value: DVGASetting) -> Self {
        value as Self
    }
}
