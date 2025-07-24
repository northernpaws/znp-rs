pub mod command;

use bit_struct::*;

/// Type of a command ID.
/// 
/// This is the second byte in a command header.
pub type ID = u8;

bit_struct! {
    // u8 is the base storage type. This can be any multiple of 8
    pub struct Command(u16) {
        // 2 bits unsigned number
        kind: command::Type,
        
        // 4-bit unsigned number
        subsytem: command::Subsystem,
        
        /// 8-bit unsigned number
        id: ID,
    }
}

/// Defines a trait for reading command header information.
pub trait CommandHeader {
    type U8Error;

    /// Returns the first byte of the command header.
    fn cmd0(&self) -> Result<u8, Self::U8Error>;

    // Returns the subsystem for the command from the first byte in bits [4:0].
    fn subsystem (&self) -> Result<command::Subsystem, Self::U8Error>;

    /// Returns the command ID (second byte) of the header.
    fn id(&self) -> Result<u8, Self::U8Error>;
}

/// Adds support for reading a u16 as a command frame header.
impl CommandHeader for u16 {
    type U8Error = <u16 as TryInto<u8>>::Error;

    fn cmd0(&self) -> Result<u8, Self::U8Error> {
        // Extracts the first byte from the two-byte command header.
        //
        // This byte contains the command type and subsystem.
        (self & 0x00FF).try_into()
    }

    fn subsystem (&self) -> Result<command::Subsystem, Self::U8Error> {
        // Convenience so we're not repeating command::Subsystem::x 31 times.
        use command::Subsystem::*;

        // Extract the first byte of the command header
        // that contains the command type and subsystem.
        let cmd0 = self.cmd0()?;

        // Compare bits [4:0] from the first byte of the command header.
        Ok(match cmd0 & 0x1F {
            0 => RPCErrorInterface,
            1 => SYSInterface,
            2 => Reserved2,
            3 => Reserved3,
            4 => AFInterface,
            5 => ZDOInterface,
            6 => SimpleAPIInterface,
            7 => UTILInterface,
            8 => Reserved8,
            9 => APPInterface,
            10 => Reserved10,
            11 => Reserved11,
            12 => Reserved12,
            13 => Reserved13,
            14 => Reserved14,
            15 => Reserved15,
            16 => Reserved16,
            17 => Reserved17,
            18 => Reserved18,
            19 => Reserved19,
            20 => Reserved20,
            21 => Reserved21,
            22 => Reserved22,
            23 => Reserved23,
            24 => Reserved24,
            25 => Reserved25,
            26 => Reserved26,
            27 => Reserved27,
            28 => Reserved28,
            29 => Reserved29,
            30 => Reserved30,
            31 => Reserved31,
            32_u8 | 32_u8..=u8::MAX => todo!()
        })
    }

    fn id(&self) -> Result<u8, Self::U8Error>  {
        (self >> 8).try_into()
    }
}

/// A packet frame that can be sent to a Z-Stack device.
pub struct Frame<const N: usize> {
    /// The length of the frame.
    /// 
    /// Can range from 0 to 250.
    length: u8,

    /// The command header for the frame.
    command: Command,

    /// The data for the frame.
    data: [u8; N]
}

/// An error code that can be returned from an SREQ as part of the SRSP response.
#[repr(u8)]
enum SRSPErrorCode {
    Ok = 0x00,
    InvalidSubsystem = 0x01,
    InvalidCommandID = 0x02,
    InvalidParameter = 0x03,
    InvalidLength = 0x04
}
