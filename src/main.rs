#[macro_use]
extern crate error_chain;

use std::net::Ipv4Addr;

use log::{error, debug};

use netif::{Runnable, ShmPacket, UdpSocket, Socket};
use netif::client::AppContext;

type Addr = (Ipv4Addr, u16);

error_chain! {
    links {
        NetIf(::netif::Error, ::netif::ErrorKind);
    }
}

fn main() -> Result<()> {
    pretty_env_logger::init();

    debug!("starting up...");

    let cb = move |sock: &mut UdpSocket, pkts: &[ShmPacket], addr, port| {
        debug!("processing batch of {} packets from {}:{}", pkts.len(), addr, port);

        if let Err(e) = sock.send_to(pkts, (addr, port)) {
            error!("failed to echo packets: {}", e);
        }
    };

    let mut ctx = AppContext::new()?;
    let handle = ctx.bind("127.0.0.1".parse().unwrap(), 9090, cb)?;

    debug!("bound socket with handle {}", handle);

    ctx.run()?;

    debug!("context finished executing");

    Ok(())
}
