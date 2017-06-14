mod proto;
mod codec;
mod service;
use tokio_proto::TcpServer;

pub fn main() {
    let mut config = super::helper::get_config();
    config.set_default("address", "0.0.0.0:143").unwrap();

    let addr = config.get_str("address").unwrap().parse().unwrap();

    // The builder requires a protocol and an address
    let server = TcpServer::new(proto::LineProto, addr);
    println!("Listening on: {}", addr);

    // We provide a way to *instantiate* the service for each new
    // connection; here, we just immediately return a new instance.
    server.serve(|| Ok(service::Echo));
}
