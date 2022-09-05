extern crate core;

use std::env;
use std::fs;
use std::ops::Add;
mod buscaminas;
mod interaccion_usuario;

use crate::interaccion_usuario::mostrar_mapa;
use buscaminas::descubrir_bombas;
use interaccion_usuario::dar_bienvenida;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let mut input = fs::read_to_string(path).expect("[ERROR] No se pudo leer el archivo.\n");

    input = input.add("\n"); // para que la cantidad de \n sea igual a la de filas

    dar_bienvenida();
    mostrar_mapa(&input);
    //renombrar bombas a minas
    let output = descubrir_bombas(input);
    mostrar_mapa(&output);
    fs::write("mapa_output.txt", output).expect("[ERROR] No se pudo escribir el archivo.\n");
}