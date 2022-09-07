//! Modulo encargado de hacer mas amigable la interaccion con el programa,
//! buscando mantener informado al usuario.

/// Breve bienvenida al programa evidenciando que inició correctamente
pub fn dar_bienvenida() {
    println!("\nBIENVENIDOS AL BUSCAMINAS!!!\n");
    println!("Con el siguiente programa vas a poder conocer las ubicaciones de las minas \
    del mapa.\n");
}

/// Funcion que se encarga de recibir un mapa y mostrarlo por pantalla.
/// Ejemplos de su visualización:
/// ```
/// ..*
/// ...
/// ```
/// ```
/// .1*
/// .11
/// ```
pub fn mostrar_mapa(mapa: &String, instancia: &str) {
    println!("Mostrando mapa {}...\n\n{}\n", instancia, mapa);
}
