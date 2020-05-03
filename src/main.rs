
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Result, Server, StatusCode};

use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

#[tokio::main]
pub async fn main() -> Result<()> {
    let addr = ([0, 0, 0, 0], 3000).into();
    let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(routes)) });
    let server = Server::bind(&addr).serve(service);

    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}

async fn routes(req: Request<Body>) -> Result<Response<Body>> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") | (&Method::GET, "/index.html") => assets("assets/index.html").await,
        (&Method::GET, path) if path.starts_with("/assets/") => assets(&path[1..]).await,
        _ => Ok(not_found())
    }
}

async fn assets(filename: &str) -> Result<Response<Body>> {
    if let Ok(file) = File::open(filename).await {
        let stream = FramedRead::new(file, BytesCodec::new());
        let body = Body::wrap_stream(stream);
        return Ok(Response::new(body));
    }

    Ok(not_found())
}

fn not_found() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body("".into())
        .unwrap()
}
