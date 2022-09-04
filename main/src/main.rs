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
//mod interaccion_usuario;

use buscaminas::jugar;
//use interaccion_usuario::dar_bienvenida;
//use interaccion_usuario::mostrar_mapa;
//use file_reader::read_file_lines;

//use ndarray::Array2;

/*fn cant_columnas(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (indice, &item) in bytes.iter().enumerate() {
        if item == b'\n' {
            return &s[0..indice];
        }
    }
    &s[..]
}*/
/*
fn cant_columnas(s: &[u8]) -> i32 {
    let mut cant_columnas = 0;
    for (indice, &item) in bytes.iter().enumerate() {
        if item == b'\n' {
            return
        }
    }
    cant_columnas
}
*/

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    let mut input = fs::read_to_string(path)
        .expect("[ERROR] No se pudo leer el archivo.\n");

    // input: ".*.*.\n..*..\n..*..\n.....\n"
    input = input.add("\n"); // para que la cantidad de \n sea igual a la de filas
    jugar(input.as_bytes());

    // bytes: [46, 42, 46, 42, 46, 10, 46, 46, 42, 46, 46, 10, 46, 46, 42, 46, 46, 10, 46, 46, 46, 46, 46]
    /*
    let cant_columnas = cant_columnas(bytes);

    while bytes.len() > 0 {

    }

    println!("fila:{:?}", fila);

    //remuevo fila len +1 por el enter
    input.remove(0);
    input.remove(0);
    input.remove(0);
    input.remove(0);
    input.remove(0);
    input.remove(0);

*/
    /*

    la funcion que tengo no me va a servir para la ultima fila porque no tiene \n
    opc1-> agregarle un \n al final (la mas facil)
    opc2-> solo obtener la primera vez la cant y voy usando eso como medida y eliminando esa cant hasta que quede vacio

    */
/*
    let fila2 = cant_columnas(&input);
    println!("fila:{:?}", fila2);


    let mut variable = String::new();
    variable.remove(0);
*/
    /*let mut mapa2: [Box<[u8]>; &cant_filas] = [
        Box::new([1, 2, 3]),
        Box::new([4, 4, 5]),
        Box::new([5, 6, 9])
    ];*/

    //println!("matriz {:?}", mapa);

    /*

    let x: [Box<[u8]>; 3] = [
    Box::new([1, 2, 3]),
    Box::new([4]),
    Box::new([5, 6])
    ];
    let y: &[Box<[u8]>] = &x;

    */


    //dar_bienvenida();
    //mostrar_mapa(&"inicial".to_string(), &mapa);
    //jugar(&mapa);
//    mostrar_mapa(&"final", &mapa);
}


/*
let s = String::from("hello");

let len = s.len();

let slice = &s[3..len];
*/

// to return a string slice:
// &str es una referencia inmutable.

/*
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}*/