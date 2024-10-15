use actix_web::{get, middleware::Logger, web::{self, Html}, App, HttpServer};

#[get("/")]
async fn index() -> Html {
    Html::new(include_str!("static/index.html"))
}

#[get("/about")]
async fn about() -> Html {
    Html::new(include_str!("static/about.html"))
}

#[get("/passgen")]
async fn passgen() -> Html {
    Html::new(include_str!("static/passgen.html"))
}

#[get("/howto")]
async fn howto() -> Html {
    Html::new(include_str!("static/howto.html"))
}

#[get("/recommendations")]
async fn recomendations() -> Html {
    Html::new(include_str!("static/recommendations.html"))
}

#[get("/images/background")]
async fn background() -> &'static [u8] {
    include_bytes!("static/background.png")
}

#[get("/images/favicon")]
async fn favicon() -> &'static [u8] {
    include_bytes!("static/favicon.png")
}

async fn handle_404() -> Html {
    Html::new(include_str!("static/404.html"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init(); // To make logs visible run app with RUST_LOG=info enviornment variable
    let ip = "0.0.0.0"; // To make webserver global write 0.0.0.0 instead of 127.0.0.1
    let port = 8080; // Change it if needed
    println!("Running server on {ip}:{port}...");
    HttpServer::new(||
          App::new()
            .service(index)
            .service(about)
            .service(passgen)
            .service(howto)
            .service(recomendations)
            .service(background)
            .service(favicon)
            .wrap(Logger::default())
            .default_service(
                web::to(handle_404)
                )
            )
        .bind((ip, port))? 
        .run()
        .await
}
