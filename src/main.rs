#[macro_use]
extern crate nickel;

use std::env;

use nickel::Nickel;

fn main() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |_req, _res| {
            "Meu microservico Rust esta online!"
        }
    });

    match env::var("PORT") {
        Ok(port) => match server.listen(format!("127.0.0.1:{}", port)) {
            Ok(_) => println!("Servidor iniciado na porta {}!", port),
            Err(_) => eprintln!("Erro ao iniciar o servidor na porta {}!", port),
        },
        Err(e) => match server.listen("127.0.0.1:8080") {
            Ok(_) => println!("Servidor iniciado na porta padrão 8080!"),
            Err(_) => eprintln!("Erro ao iniciar o servidor na porta padrão 8080!"),
        },
    };
}
