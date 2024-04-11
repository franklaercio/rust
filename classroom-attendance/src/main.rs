use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
use std::net::SocketAddr;

async fn handle_request(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&hyper::Method::GET, "/") => {
            Ok(Response::new(Body::from("Hello from GET!")))
        }
        (&hyper::Method::POST, "/") => {
            Ok(Response::new(Body::from("Hello from POST!")))
        }
        _ => {
            let response = Response::builder()
                .status(404)
                .body(Body::from("Not found"))
                .unwrap();
            Ok(response)
        }
    }
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle_request))
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Starting server on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("Can not possible start server: {}", e);
    }
}
