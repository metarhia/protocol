extern crate mhp;
extern crate tokio_proto;

use tokio_proto::TcpServer;
use mhp::*;

fn main() {
    let addr = "0.0.0.0:7878".parse().unwrap();
    let server = TcpServer::new(MhpProto, addr);

    server.serve(|| Ok(EchoService));
}
