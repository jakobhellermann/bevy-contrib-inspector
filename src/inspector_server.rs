use crossbeam_channel::{unbounded as channel, Receiver};
use std::net::TcpListener;
use tiny_http::{Response, Server};

type Event = ();

pub struct InspectorServer {
    pub rx: Receiver<Event>,
}

impl InspectorServer {
    pub fn start_in_background(addr: &str) -> Result<Self, std::io::Error> {
        let (tx, rx) = channel();

        let listener = TcpListener::bind(addr)?;

        std::thread::spawn(move || {
            for stream in listener.incoming() {
                let _stream = stream.unwrap();
                tx.send(()).unwrap();

                println!("Connection established!");
            }
        });

        Ok(InspectorServer { rx })
    }
}
