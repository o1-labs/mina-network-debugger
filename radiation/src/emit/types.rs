use std::net::{SocketAddr, IpAddr};

use super::core::Emit;

impl<W> Emit<W> for SocketAddr
where
    W: for<'a> Extend<&'a u8>,
{
    fn emit(&self, buffer: &mut W) {
        match self.ip() {
            IpAddr::V6(ip) => {
                6u8.emit(buffer);
                ip.octets().emit(buffer);
            }
            IpAddr::V4(ip) => {
                4u8.emit(buffer);
                ip.octets().emit(buffer);
            }
        }
        self.port().emit(buffer);
    }
}
