use std::env;
use std::fs;

mod buscaminas;
mod interaccion_usuario;

use buscaminas::jugar;
use interaccion_usuario::dar_bienvenida;
use interaccion_usuario::mostrar_mapa;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let mapa = fs::read_to_string(path)
        .expect("[ERROR] Deberias poder leer este archivo :(");

    dar_bienvenida();
    mostrar_mapa(&"inicial".to_string(), &mapa);
    jugar(&mapa);
//    mostrar_mapa(&"final", &mapa);
}