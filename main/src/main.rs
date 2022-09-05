extern crate core;

use std::env;
use std::fs;
use std::ops::Add;
mod buscaminas;
mod interaccion_usuario;

use buscaminas::jugar;
use interaccion_usuario::dar_bienvenida;
use crate::interaccion_usuario::mostrar_mapa;


fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let mut input = fs::read_to_string(path)
        .expect("[ERROR] No se pudo leer el archivo.\n");

    // input: ".*.*.\n..*..\n..*..\n.....\n"
    input = input.add("\n"); // para que la cantidad de \n sea igual a la de filas
    // bytes: [46, 42, 46, 42, 46, 10, 46, 46, 42, 46, 46, 10, 46, 46, 42, 46, 46, 10, 46, 46, 46, 46, 46]

    dar_bienvenida();
    mostrar_mapa(&input);
    println!("input en bytes: {:?}", input.as_bytes());
    jugar(input);// cambiar nombre -> no estamos jugando


    //mostrar_mapa(&"inicial".to_string(), &mapa);
    //jugar(&mapa);
//    mostrar_mapa(&"final", &mapa);
}

