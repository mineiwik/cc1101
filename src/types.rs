/// Radio operational mode.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum RadioMode {
    Idle,
    Sleep,
    Calibrate,
    Transmit,
    Receive,
}

/// Packet length configuration.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PacketLength {
    /// Set packet length to a fixed value.
    Fixed(u8),
    /// Set upper bound of variable packet length.
    Variable(u8),
    /// Infinite packet length, streaming mode.
    Infinite,
}

/// Address check configuration.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AddressFilter {
    /// No address check.
    Disabled,
    /// Address check, no broadcast.
    Device(u8),
    /// Address check and 0 (0x00) broadcast.
    DeviceLowBroadcast(u8),
    /// Address check and 0 (0x00) and 255 (0xFF) broadcast.
    DeviceHighLowBroadcast(u8),
}

/// Address check configuration.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Power {
    Power10Dbm = 0xC5,
    Power5Dbm = 0x86,
    Power0Dbm = 0x50,
}

/// GDO0 configuration.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Gdo0Cfg {
    /// Asserts when sync word has been sent / received, and de-asserts at the end of the packet. In RX, the pin will also de-assert when a packet is discarded due to address or maximum length filtering or when the radio enters RXFIFO_OVERFLOW state. In TX the pin will de-assert if the TX FIFO underflows
    SyncWord = 0x06,
}

/// Sync word configuration.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SyncMode {
    /// No sync word.
    Disabled,
    /// Match 15 of 16 bits of given sync word.
    MatchPartial(u16),
    /// Match 30 of 32 bits of a repetition of given sync word.
    MatchPartialRepeated(u16),
    /// Match 16 of 16 bits of given sync word.
    MatchFull(u16),
}
