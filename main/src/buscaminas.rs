//mover a otro lado el struct y la imple (creo)
#[derive(Debug)]
struct Buscaminas {
    mapa: Vec<Vec<u8>>,
    cant_filas: usize,
    cant_columnas: usize,
}

impl Buscaminas {
    pub fn new(cant_filas: usize, cant_columnas: usize) -> Self {
        Buscaminas {
            mapa: vec![vec![0; cant_columnas]; cant_filas],
            cant_filas,
            cant_columnas,
        }
    }
}

fn cant_filas(bytes: &[u8]) -> usize {
    let mut cant_filas = 0;

    for caracter in bytes {
        if *caracter == 10 {
            cant_filas += 1;
        }
    }
    cant_filas
}

fn cant_columnas(cant_bytes_matriz: usize, filas: usize) -> usize {
    (cant_bytes_matriz / filas) - 1
}

fn crear_mapa_input(input: &[u8], buscaminas: Buscaminas) -> Buscaminas {
    buscaminas
}

pub fn jugar(input: &[u8]){
    let cant_filas = cant_filas(input);
    let cant_columnas = cant_columnas(input.len(), cant_filas);

    let mut buscaminas = Buscaminas::new(cant_filas, cant_columnas);

    buscaminas = crear_mapa_input(input, buscaminas);
    //agregar_recuento_de_minas(mapa);

    println!("{:?}", buscaminas);
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