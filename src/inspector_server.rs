use crossbeam_channel::{unbounded as channel, Receiver, Sender};
use tiny_http::{Method, Request, Response, Server, StatusCode};

type Event = (String, String);

pub struct InspectorServer {
    pub rx: Receiver<Event>,
    pub handle: std::thread::JoinHandle<()>,
}

type Error = Box<dyn std::error::Error + Sync + Send>;

#[derive(Clone)]
pub struct ServerConfig {
    html: String,
}

impl ServerConfig {
    pub fn new(html: String) -> Self {
        ServerConfig { html }
    }
}

fn handle_request(
    config: &ServerConfig,
    mut req: Request,
    tx: &Sender<Event>,
) -> Result<(), std::io::Error> {
    match req.method() {
        Method::Get => return handle_get(config, req),
        Method::Put => {
            if let Some(event) = parse_body(&mut req)? {
                tx.send(event).unwrap();
            }
        }
        _ => {}
    }
    req.respond(Response::new_empty(StatusCode(200)))?;
    Ok(())
}

fn handle_get(config: &ServerConfig, req: Request) -> Result<(), std::io::Error> {
    let content_type =
        tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap();

    let mut response = Response::from_string(&config.html);
    response.add_header(content_type);
    req.respond(response)
}
fn parse_body(req: &mut Request) -> Result<Option<(String, String)>, std::io::Error> {
    let mut buf = Vec::with_capacity(req.body_length().unwrap_or_default());
    let reader = req.as_reader();
    reader.read_to_end(&mut buf)?;

    let invalid_data = |e| std::io::Error::new(std::io::ErrorKind::InvalidData, e);

    let event = String::from_utf8(buf).map_err(invalid_data)?;

    let mut iter = event.splitn(2, ':');
    match (iter.next(), iter.next()) {
        (Some(field), Some(data)) => Ok(Some((field.to_string(), data.to_string()))),
        _ => Ok(None),
    }
}

impl InspectorServer {
    pub fn start_in_background(addr: &str, config: ServerConfig) -> Result<Self, Error> {
        let (tx, rx) = channel();

        let listener = Server::http(addr)?;

        let handle = std::thread::spawn(move || {
            for req in listener.incoming_requests() {
                if let Err(e) = handle_request(&config, req, &tx) {
                    dbg!(e);
                }
            }
        });

        Ok(InspectorServer { rx, handle })
    }
}
