use axum::{http::Error, routing::get, Router};
use tower_http::services::{ServeDir, ServeFile};

use crate::{api::{self}, random_number_handler};


pub async fn init_app_server() -> Result<(), Error> {

    let app: Router = init_router(); 

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("App server listening on: {}", listener.local_addr().unwrap() );
    axum::serve(listener, app).await.unwrap();

    Ok(())
}


fn init_router() -> Router {
    // We need to serve from the build directory itself so the relative paths are correct for 
    // the React app files.
    let react_app_dist_location = "my-react-ts-app/build";
    Router::new()
        // Route to our React app 
        // Mote tha fallback file is the SPA's root index.html, so that the server knows to send all browser click 
        // to the SPA root app (which will then route from there) vs. looking for that phyiscal file to serve at that URL.
        .nest_service("/", ServeDir::new(react_app_dist_location).not_found_service(ServeFile::new( react_app_dist_location.to_owned() + "/index.html")),)
        // Route to a random piece of dynamic generated content (simulating an API call/response)
        .route("/rando", get(random_number_handler))
        // Route to a random static html file
        .nest_service( "/other-index", ServeFile::new("index2.html"))
        .route( "/api/players", get(api::methods::get_players))
}