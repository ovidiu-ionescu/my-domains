#![deny(warnings)]

use std::convert::Infallible;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};

use config::{Config, File, FileFormat};
use std::net::SocketAddr;

use lazy_static::lazy_static;

use super::fetch_whois;
use super::render;

lazy_static! {
    static ref CONFIG: ApConfig = ApConfig::read_config();
}

struct ApConfig {
    address: String,
    domains: Vec<String>,
}

impl ApConfig {
    fn read_config() -> ApConfig {
        let mut c = Config::new();
        c.merge(File::new("settings", FileFormat::Toml).required(true)).unwrap();
        let address = c.get_str("address").unwrap();

        let domain_values = c.get_array("domains").unwrap();
        let domains = domain_values.into_iter().map(|h| h.into_str().unwrap()).collect();

        ApConfig { address, domains }
    }
}

/*
fn do_work() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let domains = fetch_whois::DomainInfo::fetch_info();
    render::render(&domains)
}
*/
async fn do_work() -> String {
    let domains = fetch_whois::DomainInfo::fetch_info(&CONFIG.domains).await;
    render::render(&domains).unwrap()
}

async fn hello(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/domains") => {
            let content = do_work().await;

            let response = Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "text/html")
                .header("server", "hyper")
                .body(Body::from(content))
                .unwrap();
            Ok(response)
        }
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

pub async fn start_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();

    // For every connection, we must make a `Service` to handle all
    // incoming HTTP requests on said connection.
    let make_svc = make_service_fn(|_conn| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        async { Ok::<_, Infallible>(service_fn(hello)) }
    });

    let socket_addr: SocketAddr = CONFIG.address.parse().expect("Unble to parse socket address");
    let server = Server::bind(&socket_addr).serve(make_svc);

    println!("Listening on http://{}", &CONFIG.address);

    server.await?;

    Ok(())
}
