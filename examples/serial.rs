extern crate bm1397_protocol;

use bm1397_protocol::command::{Command, Destination};
use bm1397_protocol::register::{ChipAddress, Registers};
use bm1397_protocol::response::{Response, ResponseType};
use std::time::Duration;

fn main() {
    // println!("Hello, world!");
    // let ports = serialport::available_ports().expect("No ports found!");
    // for p in ports {
    //     println!("{}", p.port_name);
    // }
    let mut port = serialport::new("/dev/ttyUSB0", 115_200)
        .timeout(Duration::from_millis(100))
        .open()
        .expect("Failed to open port");

    let cmd = Command::read_reg(ChipAddress::default(), Destination::All);
    println!(">> {:x?}", cmd);
    port.write_all(&cmd).expect("Write failed!");

    let mut resp: [u8; 9] = [0u8; 9];
    port.read_exact(&mut resp).expect("Found no data!");
    println!("<< {:x?}", resp);
    let reg_resp = match Response::parse(&resp).expect("Error parsing") {
        ResponseType::Reg(r) => r,
        _ => panic!("Not a Reg Response"),
    };
    let chip_addr = match reg_resp.register {
        Registers::ChipAddress(ca) => ca,
        _ => panic!("Not a ChipAddress Response"),
    };
    println!("{:x?}", chip_addr);
}
