use std::convert::Infallible;
use std::fmt::format;

use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use hyper::body::Incoming;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode, Uri};
use hyper_util::client;
use hyper_util::client::legacy::Client;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server::conn::auto::Builder;

type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Gateway listening on :3000");
    println!("Forwarding everything on :4000");

    //clienty which can take request of type<Connector,Body>-> noral http connection
    let client: Client<HttpConnector, Incoming> =
        Client::builder(TokioExecutor::new()).build(HttpConnector::new());

    loop {
        let (stream, _) = listener.accept().await.unwrap();

        let io = TokioIo::new(stream);
        let client = client.clone();

        tokio::spawn(async move {
            let client = client.clone();

            let service = service_fn(move |req| proxy(req, client.clone()));

            if let Err(err) = Builder::new(TokioExecutor::new())
                .serve_connection(io, service)
                .await
            {
                eprintln!("server connection error: {:?}", err);
            }
        });
    }
}

async fn proxy(
    mut req: Request<Incoming>,
    client: Client<HttpConnector, Incoming>,
) -> Result<Response<BoxBody>, Infallible> {
    // -> uri to backend
    let path = req
        .uri()
        .path_and_query()
        .map(|x| x.as_str())
        .unwrap_or("/");
    let uri_string = format!("https://127.0.0.1:4000{}", path);

    Ok()
}
