use axum::{extract::Query, response::Html, routing::get, Router};
use tower_http::services::{ServeDir, ServeFile};
use rand::{thread_rng, Rng};
use serde::Deserialize;

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
    Router::new()
        // Route to our React app
        .nest_service("/", ServeDir::new("my-react-ts-app/build").not_found_service(ServeFile::new("index.html")),)
        // Route to a random piece of dynamic generated content (simulating an API call/response)
        .route("/rando", get(random_number_handler))
        // Route to a random static html file
        .nest_service( "/other-index", ServeFile::new("index2.html"))
}


// `Deserialize` need be implemented to use with `Query` extractor.
#[derive(Deserialize)]
struct RangeParameters {
    start: usize,
    end: usize,
}

async fn random_number_handler(Query(range): Query<RangeParameters>) -> Html<String> {
    // Generate a random number in range parsed from query.
    let random_number = thread_rng().gen_range(range.start..range.end);

    // Send response in html format.
    Html(format!("<h1>Random Number: {}</h1>", random_number))
}