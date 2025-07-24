use bit_struct::*; 

enums! {
    /// Specifies the type of a command being set.
    /// 
    /// Occupies bits [7:5] in the first byte of a command header.
    pub Type {
        /// Not supported on for CC2630 radios.
        POLL, // = 0,

        /// A synchronous request that requires an immediate response,
        /// such as a function call with a retuirn value.
        SREQ, // = 1,

        /// An asyncronous request with no return value.
        AREQ, // = 2,

        /// A synchronous response sent in response to an SREQ command.
        /// 
        /// SRSP commands subsystem and IDs are set to the same values as the corresponding SREQ command.
        /// 
        /// SRSPs should normally be non-zero, so a SRSP frame with a length of zero typically indicates an error.
        SRSP, // = 3,

        Reserved4, // = 4,
        Reserved5, // = 5,
        Reserved6, // = 6,
        Reserved7, // = 7
    }
}

enums! {
    /// Specifies the subsystem a command is for.
    /// 
    /// Occupies bits [4:0] in the first byte of the command header.
    pub Subsystem {
        RPCErrorInterface, // = 0,
        SYSInterface, // = 1,
        Reserved2, // = 2,
        Reserved3, // = 3,
        AFInterface, // = 4,
        ZDOInterface, // = 5,
        SimpleAPIInterface, // = 6,
        UTILInterface, // = 7,
        Reserved8, // = 8,
        APPInterface, // = 9,
        Reserved10, // = 10,
        Reserved11, // = 11,
        Reserved12, // = 12,
        Reserved13, // = 13,
        Reserved14, // = 14,
        Reserved15, // = 15,
        Reserved16, // = 16,
        Reserved17, // = 17,
        Reserved18, // = 18,
        Reserved19, // = 19,
        Reserved20, // = 20,
        Reserved21, // = 21,
        Reserved22, // = 22,
        Reserved23, // = 23,
        Reserved24, // = 24,
        Reserved25, // = 25,
        Reserved26, // = 26,
        Reserved27, // = 27,
        Reserved28, // = 28,
        Reserved29, // = 29,
        Reserved30, // = 30,
        Reserved31, // = 31,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Ensures that the bit-struct type is mapping the 
    // enum fields to the expected numberical values.
    #[test]
    fn test_command_type_values() {
        assert!(Type::POLL.inner_raw() == 0);
        assert!(Type::SREQ.inner_raw() == 1);
        assert!(Type::AREQ.inner_raw() == 2);
        assert!(Type::SRSP.inner_raw() == 3);

        assert!(Type::Reserved4.inner_raw() == 4);
        assert!(Type::Reserved5.inner_raw() == 5);
        assert!(Type::Reserved6.inner_raw() == 6);
        assert!(Type::Reserved7.inner_raw() == 7);
    }

    // Ensures that the bit-struct type is mapping the 
    // enum fields to the expected numberical values.
    #[test]
    fn test_command_subsystem_values() {
        assert!(Subsystem::RPCErrorInterface.inner_raw() == 0);
        assert!(Subsystem::SYSInterface.inner_raw() == 1);
        assert!(Subsystem::Reserved2.inner_raw() == 2);
        assert!(Subsystem::Reserved3.inner_raw() == 3);
        assert!(Subsystem::AFInterface.inner_raw() == 4);
        assert!(Subsystem::ZDOInterface.inner_raw() == 5);
        assert!(Subsystem::SimpleAPIInterface.inner_raw() == 6);
        assert!(Subsystem::UTILInterface.inner_raw() == 7);
        assert!(Subsystem::Reserved8.inner_raw() == 8);
        assert!(Subsystem::APPInterface.inner_raw() == 9);
        assert!(Subsystem::Reserved10.inner_raw() == 10);
        assert!(Subsystem::Reserved11.inner_raw() == 11);
        assert!(Subsystem::Reserved12.inner_raw() == 12);
        assert!(Subsystem::Reserved13.inner_raw() == 13);
        assert!(Subsystem::Reserved14.inner_raw() == 14);
        assert!(Subsystem::Reserved15.inner_raw() == 15);
        assert!(Subsystem::Reserved16.inner_raw() == 16);
        assert!(Subsystem::Reserved17.inner_raw() == 17);
        assert!(Subsystem::Reserved18.inner_raw() == 18);
        assert!(Subsystem::Reserved19.inner_raw() == 19);
        assert!(Subsystem::Reserved20.inner_raw() == 20);
        assert!(Subsystem::Reserved21.inner_raw() == 21);
        assert!(Subsystem::Reserved22.inner_raw() == 22);
        assert!(Subsystem::Reserved23.inner_raw() == 23);
        assert!(Subsystem::Reserved24.inner_raw() == 24);
        assert!(Subsystem::Reserved25.inner_raw() == 25);
        assert!(Subsystem::Reserved26.inner_raw() == 26);
        assert!(Subsystem::Reserved27.inner_raw() == 27);
        assert!(Subsystem::Reserved28.inner_raw() == 28);
        assert!(Subsystem::Reserved29.inner_raw() == 29);
        assert!(Subsystem::Reserved30.inner_raw() == 30);
        assert!(Subsystem::Reserved31.inner_raw() == 31);
        
    }
}
