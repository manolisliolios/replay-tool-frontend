pub mod html_output;

use std::{env, net::SocketAddr};
use include_dir::{include_dir, Dir};
use warp::Filter;

const REACT_FILES: Dir = include_dir!("replay-frontend");

#[tokio::main]
async fn main() {
    let current_dir = env::current_dir().expect("Failed to get current directory");

    // Specify the directory path for static files.
    let static_files_dir = current_dir.join("dist/");
    let uploads_dir = current_dir.join("uploads/");

    println!("{}", static_files_dir.display());
    println!("{}", uploads_dir.display());

    // Define the warp filter to serve static files.
    // let index_route = warp::fs::dir(static_files_dir).or(warp::fs::file("dist/index.html"));

    let upload_route = warp::path("uploads").and(warp::fs::dir(uploads_dir));
    
    // Define the route to serve any file from the embedded React files.
    // Define the route to serve any file from the embedded React files.
    let react_route = warp::path::tail().map(|file_path: warp::path::Tail| {
        let file_name = file_path.as_str().trim_start_matches('/');

        println!("Requested file: {}", file_name);

        if let Some(file) = REACT_FILES.get_file(file_name) {
            warp::reply::with_status(file.contents().to_vec(), warp::http::StatusCode::OK)
        } else {
            // If the requested file is not found, default to serving index.html.
            if let Some(index_html) = REACT_FILES.get_file("index.html") {
                warp::reply::with_status(index_html.contents().to_vec(), warp::http::StatusCode::OK)
            } else {
                warp::reply::with_status("Index.html not found".into(), warp::http::StatusCode::NOT_FOUND)
            }
        }
    });
    

    let routes = upload_route.or(react_route);

    // Specify the address to bind to with a random port.
    let addr: SocketAddr = "127.0.0.1:0".parse().expect("Failed to parse address");

    let (url, server) = warp::serve(routes).bind_ephemeral(addr);

    println!("Server started at http://{}", url);
    // Start the warp server.
    server.await;
}
