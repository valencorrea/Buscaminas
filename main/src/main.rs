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
//! *Ejemplo: cargo run mapas/mapa_input.txt*
//!
//! Otros comandos de interes:
//! - *cargo test*
//! - *cargo fmt*
//! - *cargo clippy*
//! - *cargo doc --open*

use std::env;

mod archivos_service;
mod buscaminas_service;
mod calculadora_service;
mod interaccion_usuario;

use archivos_service::escribir_archivo;
use archivos_service::leer_archivo;
use archivos_service::tiene_caracteres_validos;
use buscaminas_service::descubrir_minas;
use interaccion_usuario::dar_bienvenida;
use interaccion_usuario::mostrar_mapa;

use crate::archivos_service::ErrorArchivo;
use crate::ErrorArchivo::ErrorLectura;

/// Funcion principal que controla el flujo del programa relacionandose con
/// diferentes modulos.
/// Recibe por parametros la ruta del archivo contenedor del mapa.
/// Escribe en un nuevo archivo su resolución.
fn main() -> Result<(), ErrorArchivo> {
    let args: Vec<String> = env::args().collect();

    let input = match leer_archivo(&args[1]) {
        Ok(resultado_lectura) => resultado_lectura,
        Err(error) => return Err(error),
    };
    if !tiene_caracteres_validos(&input) {
        println!("Error leyendo el archivo. Por favor vuelva a intentar.\n");
        return Err(ErrorLectura(String::from("Error leyendo el archivo")));
    };

    dar_bienvenida();
    mostrar_mapa(&input, "input");
    let output = descubrir_minas(input);
    mostrar_mapa(&output, "output");

    match escribir_archivo("mapas/mapa_output.txt", output) {
        Ok(resultado_escritura) => resultado_escritura,
        Err(error) => {
            println!("Error escribiendo archivo.\n");
            return Err(error);
        }
    }
    Ok(())
}
