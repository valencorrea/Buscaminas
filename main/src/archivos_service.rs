//! Modulo que se centra en las funcionalidades referentes a la interacción
//! con los archivos.
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::fs::File;
use std::io::Read;
use std::ops::Add;

//#[derive(Debug)]
pub enum ErrorArchivo {
    ErrorLectura,
    ErrorEscritura,
}

//impl Error for ErrorArchivo {}

/// Funcion que recibe una ruta donde se encuentra el archivo con el mapa input, y
/// retorna el mapa leido en formato de String.
///
/// Ejemplo de retorno:
/// ```
/// "..*\n.*.\n...\n"
/// ```
/// Mapa de referencia:
/// ```
/// ..*
/// .*.
/// ...
/// ```
pub fn leer_archivo(path: &str) -> Result<String, ErrorArchivo> {
    let f = File::open(path);

    let mut f = match f {
        Ok(archivo) => archivo,
        Err(_) => return Err(ErrorArchivo::ErrorLectura),
    };

    let mut leer_archivo = String::new();
    match f.read_to_string(&mut leer_archivo) {
        Ok(_) => {
            leer_archivo = leer_archivo.add("\n");
            Ok(leer_archivo)
        },
        Err(_) => return Err(ErrorArchivo::ErrorLectura),
    }
}


/// Función que recibe una ruta en donde se guardara el archivo con el respectivo
/// recuento de minas, y el contenido a escribir en formato de String.
///
/// Ejemplo de retorno:
/// ```
/// ".1*\n121\n*1."
/// ```
/// Mapas de referencia:
/// ```
/// ..*      .1*
/// ...  ->  121
/// *..      *1.
/// ```
pub fn escribir_archivo(path: &str, mut output: String) {
    output.remove(output.len() - 1);
    fs::write(path, output).expect("[ERROR] No se pudo escribir el archivo.\n");
}