use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    let app: Router = init_router(); 

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap() );
    axum::serve(listener, app).await.unwrap();
}

fn init_router() -> Router {
    // We need to serve from the build directory itself so the relative paths are correct for 
    // the React app files.
    Router::new().nest_service(
        "/", ServeDir::new("my-react-ts-app/build")
       .not_found_service(ServeFile::new("index.html")),)
    
}
