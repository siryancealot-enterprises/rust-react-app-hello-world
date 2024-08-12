

use axum::{extract::Query, http::Error, response::Html, routing::{get, post}, Router};
use serde::Deserialize;
use tower_http::services::{ServeDir, ServeFile};
use rand::{thread_rng, Rng};

use crate::api::{self, methods::PLAYERS_API};


pub async fn init_app_server() -> Result<(), Error> {

    let app: Router = init_router(); 

    let listener = tokio::net::TcpListener::bind(std::env::var("APP_SERVER_URL").expect("APP_SERVER_URL must be set in .env file.")).await.unwrap();
    println!("App server listening on: {}", listener.local_addr().unwrap() );
    axum::serve(listener, app).await.unwrap();

    Ok(())
}


fn init_router() -> Router {
    // We need to serve from the build directory itself so the relative paths are correct for 
    // the React app files.
    let binding = std::env::var("REACT_APP_DIST_DIR").expect("REACT_APP_DIST_DIR must be set in .env file.");
    let react_app_dist_location: &str = binding.as_str();
    let path_with_index_html = react_app_dist_location.to_string() + "/index.html";
    Router::new()
        // Route to our React app 
        // Mote tha fallback file is the SPA's root index.html, so that the server knows to send all browser click 
        // to the SPA root app (which will then route from there) vs. looking for that phyiscal file to serve at that URL.
        .nest_service("/", ServeDir::new(react_app_dist_location).not_found_service(ServeFile::new( path_with_index_html)),)
        // Route to a random piece of dynamic generated content (simulating an API call/response)
        .route("/rando",get(random_number_handler))
        // Route to a random static html file
        .nest_service( "/other-index", ServeFile::new("index2.html"))
        .route( PLAYERS_API, get(api::methods::get_players))
        .route( PLAYERS_API, post(api::methods::add_player))
}



// `Deserialize` need be implemented to use with `Query` extractor.
#[derive(Debug, Deserialize)]
struct RangeParameters {
    start: usize,
    end: usize,
}

async fn random_number_handler(Query(range): Query<RangeParameters>) -> Html<String> {
    // Generate a random number in range parsed from query.
    let random_number = thread_rng().gen_range(range.start..range.end);

    // Send response in html format.
    Html(format!("<h1>Random Number: {}</h1></br><a href='/'>Go Back</a>", random_number))
}