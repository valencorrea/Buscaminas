//! Modulo que se centra en las funcionalidades referentes a la interacción
//! con los archivos.
use std::fs;
use std::fs::File;
use std::io::Read;
use std::ops::Add;

pub const BOMBA_U8: u8 = 42;
pub const PUNTO_U8: u8 = 46;

pub enum ErrorArchivo {
    ErrorLectura(String),
    ErrorEscritura(String),
}

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
        Err(error) => return Err(ErrorArchivo::ErrorLectura(error.to_string())),
    };

    let mut leer_archivo = String::new();
    match f.read_to_string(&mut leer_archivo) {
        Ok(_) => {
            leer_archivo = leer_archivo.add("\n");
            Ok(leer_archivo)
        }
        Err(error) => Err(ErrorArchivo::ErrorLectura(error.to_string())),
    }
}

/// Funcion que recibe el input leido y retorna si el archivo es valido,
/// comprobando si contiene solamente los caracteres permitidos.
pub fn tiene_caracteres_validos(input: &String) -> bool {
    let mut caracteres_invalidos = false;
    for caracter in input.as_bytes() {
        if (*caracter == BOMBA_U8) || (*caracter == PUNTO_U8) {
            caracteres_invalidos = true;
        }
    }
    caracteres_invalidos
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
pub fn escribir_archivo(path: &str, mut output: String) -> Result<(), ErrorArchivo> {
    output.remove(output.len() - 1);
    fs::write(path, output).map_err(|error| ErrorArchivo::ErrorEscritura(error.to_string()))
}
