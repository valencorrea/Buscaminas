extern crate core;

use std::env;
use std::fs;
use std::fmt;
use std::fs::File;
//use std::io::BufReader;
use std::io::prelude::*;
//mod file_io::file_reader as files;
use std::io::{BufRead, BufReader, Bytes};
//mod buscaminas;
//mod interaccion_usuario;

//use buscaminas::jugar;
//use interaccion_usuario::dar_bienvenida;
//use interaccion_usuario::mostrar_mapa;
//use file_reader::read_file_lines;

//use ndarray::Array2;

fn obtenerFila(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b'\n' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];

    // Retorna el siguiente string
    // ".*.*.\n..*..\n..*..\n....."

    let mut input = fs::read_to_string(path)
        .expect("[ERROR] No se pudo leer el archivo.\n");

    // bytes [46, 42, 46, 42, 46, 10, 46, 46, 42, 46, 46, 10, 46, 46, 42, 46, 46, 10, 46, 46, 46, 46, 46]
    let bytes = input.as_bytes();
    //println!("bytes {:?}", bytes);

    //let bytes = input.as_bytes();
    //let mut i = 0;
    //while (bytes.get(i).eq(b'\n')) && (i<bytes.len()){
    //    i = i+1;
    //}
    //let fila = &bytes[0..i];
    //println!("{:?}", fila);
    //let v: bool = true;

    let fila = obtenerFila(&input);

    println!("fila:{:?}, cant caracteres: {:?}", fila, fila.len());



    /*for &caracter in bytes.iter() {
        if caracter == b'\n' {
            cant_filas = cant_filas + 1;
        }
    }
    cant_filas = cant_filas + 1;
    println!("cant filas: {}", cant_filas);

    let cant_filas_matriz: usize = cant_filas;

    let mapa = Box::new(cant_filas);*/


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