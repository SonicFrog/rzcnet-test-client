#[macro_use]
extern crate error_chain;

use std::net::Ipv4Addr;
use std::sync::Arc;

use log::{debug, error};

use netif::client::AppContext;
use netif::{Runnable, ShmPacket, Socket};

type Addr = (Ipv4Addr, u16);

error_chain! {
    links {
        NetIf(::netif::Error, ::netif::ErrorKind);
    }
}

fn main() -> Result<()> {
    pretty_env_logger::init();

    debug!("starting up...");

    let cb = move |_env: &(), sock: &dyn Socket<Addr, ()>, pkts: &[ShmPacket], addr| {
        let (addr, port) = addr;
        debug!(
            "processing batch of {} packets from {}:{}",
            pkts.len(),
            addr,
            port
        );

        let mut iter = pkts.iter().map(|x| x.into());

        if let Err(e) = sock.send_to(&mut iter, (addr, port)) {
            error!("failed to echo packets: {}", e);
        }
    };

    let mut ctx = AppContext::new_with_names(netif::SOCK_PATH, 1, ())?;
    let handle = ctx.bind("127.0.0.1".parse().unwrap(), 9090, Arc::new(cb))?;

    debug!("bound socket with handle {}", handle);

    ctx.run()?;

    debug!("context finished executing");

    Ok(())
}
