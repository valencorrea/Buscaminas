fn contar_filas(bytes: &[u8]) -> usize {
    let mut cant_filas = 0;

    for caracter in bytes {
        if *caracter == 10 {
            cant_filas += 1;
        }
    }
    cant_filas
}

fn contar_columnas(cant_bytes_matriz: usize, filas: usize) -> usize {
    let mut cant_columnas = (cant_bytes_matriz / filas) - 1;
    cant_columnas
}

pub fn jugar(input: &[u8]){

    let cant_filas = contar_filas(input);
    let cant_columnas = contar_columnas(input.len(), cant_filas);
    println!("cant columnas {:?} cant filas {:?}", cant_columnas, cant_filas)

}
/*
.*.*.\n
..*..\n
..*..\n
.....\n
*/

/*
fn cant_columnas(s: &[u8]) -> i32 {
    let mut cant_columnas = 0;
    for (indice, &item) in bytes.iter().enumerate() {
        if item == b'\n' {
            return
        }
    }
    cant_columnas
}
*/