use std::env;
mod buscaminas_service;
mod interaccion_usuario;
pub mod archivos_service;

use interaccion_usuario::mostrar_mapa;
use interaccion_usuario::dar_bienvenida;
use buscaminas_service::descubrir_bombas;
use archivos_service::leer_archivo;
use archivos_service::escribir_archivo;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = leer_archivo(&args[1]);

    dar_bienvenida();
    mostrar_mapa(&input, "input");
    let output = descubrir_bombas(input);

    mostrar_mapa(&output, "output");
    escribir_archivo("mapa_output.txt", output);
}
