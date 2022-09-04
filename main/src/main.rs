extern crate core;

use std::env;
use std::fs;
use std::fmt;
use std::fs::File;
//use std::io::BufReader;
use std::io::prelude::*;
//mod file_io::file_reader as files;
use std::io::{BufRead, BufReader, Bytes};
use std::ops::Add;
mod buscaminas;
mod interaccion_usuario;

use buscaminas::jugar;
use interaccion_usuario::dar_bienvenida;
//use interaccion_usuario::mostrar_mapa;
//use file_reader::read_file_lines;

//use ndarray::Array2;


fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let mut input = fs::read_to_string(path)
        .expect("[ERROR] No se pudo leer el archivo.\n");

    // input: ".*.*.\n..*..\n..*..\n.....\n"
    input = input.add("\n"); // para que la cantidad de \n sea igual a la de filas
    // bytes: [46, 42, 46, 42, 46, 10, 46, 46, 42, 46, 46, 10, 46, 46, 42, 46, 46, 10, 46, 46, 46, 46, 46]

    dar_bienvenida();
    jugar(input.as_bytes());// cambiar nombre -> no estamos jugando


    /*let mut mapa2: [Box<[u8]>; &cant_filas] = [
        Box::new([1, 2, 3]),
        Box::new([4, 4, 5]),
        Box::new([5, 6, 9])
    ];*/

    //println!("matriz {:?}", mapa);


    //mostrar_mapa(&"inicial".to_string(), &mapa);
    //jugar(&mapa);
//    mostrar_mapa(&"final", &mapa);
}

