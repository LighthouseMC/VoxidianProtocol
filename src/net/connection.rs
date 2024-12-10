use std::net::{SocketAddr, TcpStream};
use std::ops::Deref;
use std::sync::{Arc, Mutex, MutexGuard};

pub struct Connection {
    stream: TcpStream,
    addr: SocketAddr,
}

pub struct ConnectionRef(Arc<Mutex<Connection>>);

impl Connection {
    pub fn new(stream: TcpStream, addr: SocketAddr) -> ConnectionRef {
        ConnectionRef(Arc::new(Mutex::new(Connection { stream, addr })))
    }
}
impl ConnectionRef {
    pub fn unlock(&self) -> MutexGuard<Connection> {
        self.0.lock().unwrap()
    }

    pub fn networking_loop(&self) {
        loop {
            // TODO: read 2 varints from stream and then packet data
            // then parse and process packet

            // just break for now
            break;
        }
    }
}
