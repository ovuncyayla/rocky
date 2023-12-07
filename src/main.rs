mod route;

use std::{path::PathBuf, str::FromStr};
use actix_web::{get, web, App, HttpServer, HttpResponse, guard};

use std::sync::Mutex;

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

async fn index(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {counter}") // <- response with count
}

// this function could be located in a different module
fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| async { HttpResponse::Ok().body("app") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}


fn configZZZZ(cfg: &mut web::ServiceConfig) {

    let routes = vec![
        ( "giveittome", confy("config.yaml"))
    ];

    let app_scope = web::scope("/");

    for route in routes {
        let (path, _conf) = route;
        cfg.service(
            web::resource(path)
            .route(web::get().to(|| async {
                let b = serde_json::to_string(&jg::generate_json(&confy("config.yaml")).unwrap()).unwrap();
                HttpResponse::Ok().body(b)}
            ))
        );
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Note: web::Data created _outside_ HttpServer::new closure

    // TODO: Later dudes!
    // let routes = route::RouteConfig {
    //     routes: vec![
    //         route::RouteDef { 
    //             path: "/giveittome",
    //             config_file: "config.yaml"
    //         }
    //     ]
    // };
    



    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        // move counter into the closure
        App::new()
            .app_data(counter.clone()) // <- register the created data
            .service(
                web::scope("/")
                    .guard(guard::Host("www.rust-lang.org"))
                    .route("", web::to(|| async { HttpResponse::Ok().body("www") })),
            )
            .service(
                web::scope("/")
                    .guard(guard::Host("users.rust-lang.org"))
                    .route("", web::to(|| async { HttpResponse::Ok().body("user") })),
            )
            .configure(config)
            .configure(configZZZZ)
            .service(web::scope("/somepath").configure(config))
            // .route("/", web::get().to(index))
            .route("/", web::to(HttpResponse::Ok))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn confy(path: &str) -> jg::Config {
    jg::Config::from(PathBuf::from_str(path).unwrap())
}

#[allow(unused)]
fn asd() {

    let cfg = jg::Config::from(PathBuf::from_str("config.yaml").unwrap());
    let asd = jg::generate_json(&cfg);
    println!("{:?}", asd);

}
