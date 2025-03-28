use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

use clap::builder::OsStr;
use clap::Parser;
use http_body_util::Full;
use hyper::header::HeaderValue;
use hyper::StatusCode;
use hyper::{body::Bytes, server::conn::http1, service::service_fn, Request, Response};
use hyper_util::rt::TokioIo;
use lexer::{Lexer, Project};
use log::{debug, info};
use tokio::net::TcpListener;

use nophp::compiler::Compiler;

async fn handler(
    req: Request<hyper::body::Incoming>,
    project: Arc<Project>,
) -> hyper::Result<Response<Full<Bytes>>> {
    let uri = req.uri();
    let file = uri.path().trim_start_matches('/');

    let ast = project.get(file);

    match ast {
        Some(ast) => {
            let mut buffer = String::new();
            let mut scope_vars = HashMap::new();
            let mut compiler = Compiler::new(&mut buffer, &mut scope_vars);
            let ast = ast.as_array().unwrap(); // FIXME
            compiler.execute(ast);
            compiler.run();
            info!("[{} {} 200]", req.method(), uri);
            let mut resp = Response::new(Full::new(Bytes::from(buffer)));
            resp.headers_mut().append("Content-Type", HeaderValue::from_str("text/html").unwrap());
            Ok(resp)
        }
        None => {
            let mut err = Response::new(Full::new(Bytes::from("404 Not Found")));
            *err.status_mut() = StatusCode::NOT_FOUND;
            info!("[{} {} 404]", req.method(), uri);
            Ok(err)
        }
    }
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value = "./app")]
    dir: String,
    #[arg(short, long, default_value_t = 3000)]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let args = Args::parse();

    let path = PathBuf::from(args.dir);

    if !path.is_dir() {
        return Err("Provide path is not a valid dir".into());
    }

    // make an iterator of every file in the path
    let files = std::fs::read_dir(path)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|path| path.is_file())
        .filter(|path| path.extension() == Some(&OsStr::from("php")))
        .filter_map(|path| path.to_str().map(|s| s.to_string()))
        .collect::<Vec<_>>();

    let read_files: Vec<_> = files
        .iter()
        .filter_map(|path| fs::read_to_string(path).ok())
        .collect();

    debug!("[SERVER] Found {} php files", files.len());

    let lexer = Lexer::new(&read_files);
    let ast_list = lexer.parse()?;

    let files_map: HashMap<_, _> = files
        .iter()
        .map(|f| f.trim_start_matches('.'))
        .map(|f| f.trim_start_matches('/'))
        .map(|f| f.to_string())
        .zip(ast_list.into_iter())
        .collect();

    debug!("[SERVER] Parsed {} php files", files_map.len());

    // use reference counting
    let files_map = Arc::new(files_map);

    let addr = SocketAddr::from(([127, 0, 0, 1], args.port));
    let listener = TcpListener::bind(addr).await?;

    info!("Listening on {}", addr);

    loop {
        let (stream, _) = listener.accept().await?;

        let io = TokioIo::new(stream);
        let state = files_map.clone();

        tokio::task::spawn(async move {
            let res = http1::Builder::new()
                .serve_connection(io, service_fn(|req| handler(req, state.clone())))
                .await;

            if let Err(err) = res {
                eprintln!("Error serving connection {err:?}");
            }
        });
    }
}
