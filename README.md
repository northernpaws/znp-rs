# znp-rs

This crate provides an interface between Rust and ZNP (Zigbee Network Processor) CC2531, CC13x2, and CC26x2 Texas Instruments Zigbee-Stack 3.0 (Z-Stack) based radios.

> `no_std` note: At the moment the library is designed for use running on a host with the Rust std library available, and assumes it's communicating over serial - whether that's USB serial, a COM port, TCP serial, or something else. In the future I plan to add `no_std` support for embedded use.

This crate was primarilly designed around the SMLIGHT SLZB-06M, a Zigbee coordinator that can be run over PoE, making it easy to mount on a tall wall or ceiling without requiring an additional power source.

## References

 - https://www.ti.com/tool/Z-STACK
 - https://github.com/Koenkk/Z-Stack-firmware/tree/master/coordinator/Z-Stack_3.x.0
 - https://software-dl.ti.com/simplelink/esd/plugins/simplelink_zigbee_sdk_plugin/1.60.00.14/docs/zigbee_user_guide/html/zigbee/developing_zigbee_applications/z_stack_developers_guide/z-stack-overview.html
 - https://github.com/zigpy/zigpy-znp
 - https://software-dl.ti.com/simplelink/esd/plugins/simplelink_zigbee_sdk_plugin/1.60.01.09/exports/docs/zigbee_user_guide/html/zigbee/developing_zigbee_applications/znp_interface/znp_interface.html
