use std::convert::Infallible;

use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use hyper::body::Incoming;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode, Uri, header::HeaderValue};
use hyper_util::client::legacy::Client;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::rt::{TokioExecutor, TokioIo};
use hyper_util::server::conn::auto::Builder;
use serde::Deserialize;
use std::fs;
use std::sync::atomic::{AtomicUsize, Ordering};

type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;

static ROOT_BACKEND_INDEX: AtomicUsize = AtomicUsize::new(0);

#[derive(Deserialize, Debug)]
struct Route {
    path: String,
    upstream: String,
}

#[derive(Deserialize, Debug)]
struct Config {
    routes: Vec<Route>,
}

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Gateway listening on :3000");
    println!("Forwarding everything on :4000 - 4002");

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
    let path = req.uri().path();
    let query = req
        .uri()
        .query()
        .map(|q| format!("?{q}"))
        .unwrap_or_default();

    let config = load_config();

    let mut upstream = "http://127.0.0.1:4000".to_string();
    let mut matched_prefix = "";

    if path == "/" {
        let backends = [
            "http://127.0.0.1:8000",
            "http://127.0.0.1:8001",
            "http://127.0.0.1:8002",
        ];
        let idx = ROOT_BACKEND_INDEX.fetch_add(1, Ordering::Relaxed) % backends.len();
        upstream = backends[idx].to_string();
    }

    for route in &config.routes {
        if path.starts_with(&route.path) && route.path.len() > matched_prefix.len() {
            upstream = route.upstream.clone();
            matched_prefix = &route.path;
        }
    }

    let remainder = path.strip_prefix(matched_prefix).unwrap_or(path);
    let uri_string = if remainder.is_empty() {
        format!("{}{}{}", upstream, matched_prefix, query)
    } else {
        format!("{}{}{}{}", upstream, matched_prefix, remainder, query)
    };
    //   println!("UGHHHHHHHHHHH{}", path);
    println!("Request path : {} :::: Redirecting to {}", path, uri_string);

    let uri: Uri = match uri_string.parse() {
        Ok(u) => u,
        Err(_) => return Ok(simple_response(StatusCode::BAD_REQUEST, "Invalid URI")),
    };

    *req.uri_mut() = uri;

    req.headers_mut().remove("host");
    req.headers_mut()
        .insert("x-forwarded-by", HeaderValue::from_static("rust-gateway"));

    //forward reqwest
    let backend_response = match client.request(req).await {
        Ok(res) => res,
        Err(err) => {
            eprintln!("backend error : {:?}", err);
            return Ok(simple_response(
                StatusCode::BAD_GATEWAY,
                "Backend unavailable",
            ));
        }
    };

    let (parts, body) = backend_response.into_parts();
    let body = body.boxed();

    //final response
    let mut builder = Response::builder().status(parts.status);
    for (key, value) in parts.headers.iter() {
        builder = builder.header(key, value);
    }
    let response = builder.body(body).unwrap();

    Ok(response)
}

fn simple_response(status: StatusCode, message: &'static str) -> Response<BoxBody> {
    Response::builder()
        .status(status)
        .body(
            Full::new(Bytes::from(message))
                .map_err(|never| match never {})
                .boxed(),
        )
        .unwrap()
}

fn load_config() -> Config {
    let content = fs::read_to_string("config.yaml").expect("failed to read config");

    serde_yaml::from_str(&content).expect("invalid yaml")
}
