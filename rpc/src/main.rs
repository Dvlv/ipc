extern crate xml_rpc;

use xml_rpc::{Fault, Server};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct MoveRightParams {
    pub point: Point,
    pub m: i64,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
struct Point {
    pub x: i64,
    pub y: i64,
}

fn move_right(mut p: MoveRightParams) -> Result<Point, Fault> {
    p.point.x += p.m;

    Ok(p.point)
}

fn main() {
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let mut server = Server::new();

    server.register_simple("move_right", &move_right);

    let bound_server = server.bind(&socket).unwrap();

    println!("Running server");
    bound_server.run();
}
