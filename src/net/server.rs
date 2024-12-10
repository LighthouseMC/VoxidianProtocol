use crate::net::connection::{Connection, ConnectionRef};
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener};

pub struct Server {
    listener: TcpListener,
    connections: Vec<ConnectionRef>,
}

impl Server {
    pub fn new(addr: SocketAddrV4) -> Server {
        Server {
            listener: TcpListener::bind(addr).unwrap(),
            connections: Vec::new(),
        }
    }

    pub fn do_networking(&self) {
        loop {
            let client = self.listener.accept();
            if let Ok(client) = client {
                let connection = Connection::new(client.0, client.1);

                std::thread::spawn(move || {
                    connection.networking_loop();
                });
            };
        }
    }
}
