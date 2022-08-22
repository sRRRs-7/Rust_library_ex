use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Debug)]
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

trait From<T> {
    fn from(v: T) -> Self;
}


impl From<[u8; 4]> for IpAddr {
    fn from(v: [u8; 4]) -> Self {
        Self::V4(Ipv4Addr::from(v))
    }
}

impl From<[u16; 8]> for IpAddr {
    fn from(v: [u16; 8]) -> Self {
        Self::V6(Ipv6Addr::from(v))
    }
}

pub fn main() {
    let arr: [u8; 4] = [127,0,0,1];
    let ipv4 = <IpAddr as From<[u8; 4]>>::from(arr);
    println!("{:?}", ipv4);
}