mod app;

use app::{ RouterConfig, RouteConfig };
use clap::Parser;
use log::{info, debug, warn};
use serde_yaml;
use std::{path::PathBuf, str::FromStr};
use actix_web::{http, get, web, App, HttpServer, HttpResponse, guard,};

async fn handler(data: web::Data<jg::Config>) -> String {
    let asd = jg::generate_json(&data).unwrap();
    serde_json::to_string(&asd).unwrap()
}

fn config(cfg: &mut web::ServiceConfig, routerconfig: RouterConfig) {

    for route in routerconfig.routes {
        let RouteConfig { method, path, config } = route;
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
            .app_data(web::Data::new(confy(&config)))
            .route(route.to(handler))
        );
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct AppArgs {
    /// Sets a custom config file
    #[arg(short, long, value_name = "Config File", value_parser = clap::value_parser!(PathBuf),  default_value = "config.yaml")]
    config: Option<PathBuf>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let env = env_logger::Env::default()
        .filter_or("ROCKY_LOG", "info")
        .write_style_or("ROCKY_LOG_STYLE", "always");

    env_logger::init_from_env(env);


    let args = AppArgs::parse();
    let routerconfig: RouterConfig = RouterConfig::from(args.config.unwrap());

    // Note: web::Data created _outside_ HttpServer::new closure

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
            .configure(|cfg: &mut web::ServiceConfig| {
                config(cfg, routerconfig.clone());
            })
            // .service(web::scope("/somepath").configure(config))
            // .route("/", web::get().to(index))
            .service(web::scope("/somepath"))
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

