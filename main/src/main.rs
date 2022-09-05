extern crate core;

use std::env;
use std::fs;
use std::ops::Add;
mod buscaminas;
mod interaccion_usuario;

use crate::interaccion_usuario::mostrar_mapa;
use buscaminas::jugar;
use interaccion_usuario::dar_bienvenida;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let mut input = fs::read_to_string(path).expect("[ERROR] No se pudo leer el archivo.\n");

    input = input.add("\n"); // para que la cantidad de \n sea igual a la de filas

    dar_bienvenida();
    mostrar_mapa(&input);
    println!("input en bytes: {:?}", input.as_bytes());
    jugar(input); // cambiar nombre -> no estamos jugando
}
