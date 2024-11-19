mod org_bootc;

use crate::org_bootc::VarlinkClientInterface;
use varlink::Connection;

fn main() {
    let connection = Connection::with_address("unix:/run/org.bootc.varlink").unwrap();
    let mut org_bootc_service = org_bootc::VarlinkClient::new(connection.clone());
    let reply = org_bootc_service.status().call().unwrap();
    println!("{}", reply.status);
}
