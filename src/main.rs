mod route;
use log::{info, debug, warn};
use std::{path::PathBuf, str::FromStr};
use actix_web::{http, get, web, App, HttpServer, HttpResponse, guard,};

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


async fn i(data: web::Data<jg::Config>) -> String {
    let asd = jg::generate_json(&data).unwrap();
    serde_json::to_string(&asd).unwrap()
}

fn configZZZZ(cfg: &mut web::ServiceConfig) {
    let routes = vec![
        ( "GET", "giveittome", confy("config.yaml"))
    ];

    for route in routes {
        let (method, path, conf) = route;
        println!("{}", path);
        let route: actix_web::Route = match http::Method::from_bytes(method.as_bytes()) {
            Ok(http::Method::GET) => web::get(),
            Ok(http::Method::POST) => web::post(),
            Ok(http::Method::PUT) => web::put(),
            Ok(http::Method::DELETE) => web::delete(),
            Err(err) => {
                warn!("Invalid request method: {}, {}", method, err);
                continue;
            },
            _ => web::route()
        };
        cfg.service(
            web::resource(path)
            .app_data(web::Data::new(conf))
            .route(route.to(i))
        );
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let env = env_logger::Env::default()
        .filter_or("JG_LOG", "info")
        .write_style_or("JG_LOG_STYLE", "always");

    env_logger::init_from_env(env);

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

    // let counter = web::Data::new(AppStateWithCounter {
    //     counter: Mutex::new(0),
    // });


    let routes = vec![
        ( "giveittome", confy("config.yaml"))
    ];


    HttpServer::new(move || {
        // move counter into the closure
        App::new()
            // .app_data(confy("config.yaml")) // <- register the created data
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
    .workers(1)
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
