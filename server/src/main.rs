use std::net::SocketAddr;

use clap::Parser;
use http_body_util::Full;
use hyper::StatusCode;
use hyper::{body::Bytes, server::conn::http1, service::service_fn, Request, Response};
use hyper_util::rt::TokioIo;
use nophp::lexer::lex;
use tokio::net::TcpListener;

use nophp::compiler::Compiler;
use nophp::prelude::*;

async fn handler(req: Request<hyper::body::Incoming>) -> hyper::Result<Response<Full<Bytes>>> {
    println!("[{} {}]", req.method(), req.uri());

    let ast = lex(include_str!("../../nophp.php"));

    match ast {
        Ok(ast) => {
            let mut compiler = Compiler::new();

            let ast = ast
                .as_array()
                .expect("Malformed AST Returned (AST does not start with an array)");

            compiler.execute(ast);
            compiler.run();

            Ok(Response::new(Full::new(Bytes::from("Hello World"))))
        }
        Err(err) => {
            eprintln!("[NOPHP-ERR] {err}");
            let mut error = Response::new(Full::new(Bytes::from("500 Internal Server Error")));
            *error.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            Ok(error)
        }
    }
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value="./app")]
    path: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let path = Args::parse().path;

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;

    println!("Listening on {}", addr);

    loop {
        let (stream, _) = listener.accept().await?;

        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            let res = http1::Builder::new()
                .serve_connection(io, service_fn(handler))
                .await;

            if let Err(err) = res {
                eprintln!("Error serving connection {err:?}");
            }
        });
    }
}
