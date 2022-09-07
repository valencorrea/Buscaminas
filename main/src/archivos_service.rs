//! Modulo que se centra en las funcionalidades referentes a la interacción
//! con los archivos.
//use std::io::Error;
use std::fs;
use std::ops::Add;

//devuelven un result -> manejarlo
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
pub fn leer_archivo(path: &str) -> String {
    /*let mut input = fs::read_to_string(path);// explicar en informe
    match input {
        Ok(i) => i,
        Err(mensaje_error) => return Err(mensaje_error),
    };
    //input = input.add("\n"); // para que la cantidad de \n sea igual a la de filas -> detallar en el informe
    Ok(input)*/
    let mut input = fs::read_to_string(path).expect("[ERROR] No se pudo leer el archivo.\n"); // explicar en informe
    input = input.add("\n"); // para que la cantidad de \n sea igual a la de filas -> detallar en el informe
    input
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