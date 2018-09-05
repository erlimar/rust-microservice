#[macro_use]
extern crate nickel;

use std::env;

use nickel::status::StatusCode;
use nickel::MediaType;
use nickel::Nickel;

fn main() {
    let default_port = "9090";
    let port_str = env::var("PORT").unwrap_or(String::from(default_port));
    let port: u32 = port_str.parse().unwrap();
    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |_, mut res| {
            res.set(StatusCode::Ok);
            res.set(MediaType::Json);

            "{\"mensagem\": \"Meu microservico Rust esta online!\"}"
        }
    });

    match server.listen(format!("0.0.0.0:{}", port)) {
        Ok(_) => println!("Servidor iniciado na porta {}!", port),
        Err(_) => eprintln!("Erro ao iniciar o servidor na porta {}!", port),
    };
}
