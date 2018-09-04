#[macro_use]
extern crate nickel;

use nickel::Nickel;

fn main() {
    let mut server = Nickel::new();

    server.utilize(router! {
        get "**" => |_req, _res| {
            "Meu microservico Rust esta online!"
        }
    });

    match server.listen("127.0.0.1:8080") {
        Ok(_) => println!("Servidor iniciado!"),
        Err(_) => eprintln!("Erro ao iniciar o servidor em 127.0.0.1:8080!"),
    }
}
