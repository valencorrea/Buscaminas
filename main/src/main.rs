//! # Buscaminas
//! ###### Valentina Laura Correa - vcorrea@fi.uba.ar - 104415
//! ______________
//! ##### Introducción
//! La presente entrega contiene las funcionalidades pedidas para
//! el trabajo practico nº1 de la materia Taller de Programacion I - curso Deymonnaz.
//!
//! ##### Objetivo
//! El objetivo de este trabajo practico consta de simular la logica del juego
//! [Buscaminas](https://es.wikipedia.org/wiki/Buscaminas)
//! implementandolo en
//! [Rust](https://doc.rust-lang.org/rust-by-example/index.html)
//! siguiendo los conceptos trabajados en clase.
//!
//! Para acceder al repositorio donde fue desarrollado el mismo
//! se puede visitar el siguiente [enlace](https://github.com/valencorrea/Buscaminas).
//!
//! ##### Ejecución
//! Para comenzar a utilizar el programa se deberá hacer uso del comando *cargo run* seguido
//! de la ruta en donde se encuentra el archivo de entrada.
//! En particular, los archivos de entrada estan dentro de */main/src/mapas*, ruta en
//! donde tambien se guardara el archivo de salida.
//!
//! *Ejemplo: cargo run mapa_input.txt*
//!
//! Otros comandos de interes:
//! - *cargo test*
//! - *cargo fmt*
//! - *cargo clippy*
//! - *cargo test*

use std::env;
use std::process::exit;

mod archivos_service;
mod buscaminas_service;
mod calculadora_service;
mod interaccion_usuario;

use archivos_service::escribir_archivo;
use archivos_service::leer_archivo;
use buscaminas_service::descubrir_minas;
use interaccion_usuario::dar_bienvenida;
use interaccion_usuario::mostrar_mapa;
use crate::ResultadoBuscaminas::{BuscaminasError, BuscaminasExito};

#[derive(Debug)]
pub enum ResultadoBuscaminas {
    BuscaminasError,
    BuscaminasExito,
}

/// Funcion principal que controla el flujo del programa relacionandose con
/// diferentes modulos.
/// Recibe por parametros la ruta del archivo contenedor del mapa.
/// Escribe en un nuevo archivo su resolución.
/// Se asume que el contenido del archivo llega en el formato correcto
fn main() {
    let args: Vec<String> = env::args().collect();

    let input = match leer_archivo(&args[1]){
        Ok(resultado_lectura) => {
            resultado_lectura
        }
        Err(_) => exit(-1) // buscar un similar porque no libera memoria
    };

    dar_bienvenida();
    mostrar_mapa(&input, "input");
    let output = descubrir_minas(input);

    mostrar_mapa(&output, "output");

    match escribir_archivo("mapas/mapa_output.txt", output){
        Ok(resultado_escritura) => {
            resultado_escritura
        }
        Err(_) => {
            exit(-1)
        }
    };
    exit(0)
    // agregar que se guard con el mismo nombre
}