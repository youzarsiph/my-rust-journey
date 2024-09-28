use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Actix calculator microservice.")
}

#[get("/add/{x}/{y}")]
async fn add(request: web::Path<(i32, i32)>) -> impl Responder {
    let (x, y) = request.into_inner();

    format!("{}", actix_calculator::add(x, y))
}

#[get("/sub/{x}/{y}")]
async fn sub(request: web::Path<(i32, i32)>) -> impl Responder {
    let (x, y) = request.into_inner();

    format!("{}", actix_calculator::sub(x, y))
}

#[get("/mul/{x}/{y}")]
async fn mul(request: web::Path<(i32, i32)>) -> impl Responder {
    let (x, y) = request.into_inner();

    format!("{}", actix_calculator::mul(x, y))
}

#[get("/div/{x}/{y}")]
async fn div(request: web::Path<(i32, i32)>) -> impl Responder {
    let (x, y) = request.into_inner();

    format!("{}", actix_calculator::div(x, y))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Running the server on http://127.0.0.1:8080");
    println!("Ctrl + C to stop the server.");

    HttpServer::new(|| {
        App::new()
            .service(home)
            .service(add)
            .service(sub)
            .service(mul)
            .service(div)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
