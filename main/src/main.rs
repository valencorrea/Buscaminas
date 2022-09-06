use std::env;
mod buscaminas_service;
mod interaccion_usuario;
mod calculadora_service;
mod archivos_service;

use archivos_service::escribir_archivo;
use archivos_service::leer_archivo;
use buscaminas_service::descubrir_minas;
use interaccion_usuario::dar_bienvenida;
use interaccion_usuario::mostrar_mapa;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = leer_archivo(&args[1]);

    dar_bienvenida();
    mostrar_mapa(&input, "input");
    let output = descubrir_minas(input);

    mostrar_mapa(&output, "output");
    escribir_archivo("mapas/mapa_output.txt", output); // agregar que se guarde con el mismo nombre
}
