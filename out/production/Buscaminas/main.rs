use std::env;
use std::fs;
mod file_io;
use file_io::file_reader as files;

mod buscaminas;
mod interaccion_usuario;
//mod archivos;

use buscaminas::jugar;
use interaccion_usuario::dar_bienvenida;
use interaccion_usuario::mostrar_mapa;
use crate::file_io::file_errors::CoulNotOpenFile;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    //let mapa = fs::read_to_string(path)
    //   .expect("[ERROR] Deberias poder leer este archivo :(");

    let mapa = match files::read_file_lines(path) {
        Ok(v) => v,
        _ => {}
    };



    dar_bienvenida();
    //mostrar_mapa(&"inicial".to_string(), &mapa);
    //jugar(&mapa);
//    mostrar_mapa(&"final", &mapa);
}