use std::fs;
use std::ops::Add;

//devuelven un result -> manejarlo
pub fn leer_archivo(path: &str) -> String {
    let mut input = fs::read_to_string(path).expect("[ERROR] No se pudo leer el archivo.\n");// explicar en informe
    input = input.add("\n"); // para que la cantidad de \n sea igual a la de filas -> detallar en el informe
    input
}

pub fn escribir_archivo(path: &str, mut output: String) {
    output.remove(output.len() - 1); // saco el ultimo \n para no ensuciar el archivo
    fs::write(path, output).expect("[ERROR] No se pudo escribir el archivo.\n");
}