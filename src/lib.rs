pub mod znp;

pub enum Address {
    /// 16-bit Zigbee network address
    Bit16(u16),
    /// 64-bit IEEE address
    Bit64(u64),
}


#[cfg(test)]
mod tests {
    use super::*;

}
