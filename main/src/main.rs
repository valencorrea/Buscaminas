use std::env;
mod buscaminas;
mod interaccion_usuario;
mod service_archivos;

use interaccion_usuario::mostrar_mapa;
use buscaminas::descubrir_bombas;
use interaccion_usuario::dar_bienvenida;
use service_archivos::leer_archivo;
use service_archivos::escribir_archivo;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = leer_archivo(&args[1]);

    dar_bienvenida();
    mostrar_mapa(&input, "input");
    let output = descubrir_bombas(input);

    mostrar_mapa(&output, "output");
    escribir_archivo("mapa_output.txt", output);
}