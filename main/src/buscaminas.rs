use std::io::BufRead;

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

fn quitar_enters(mut input: String, cant_filas: usize, cant_columnas: usize) -> String{
    for indice in 0..cant_filas{
        input.remove(cant_columnas*(indice+1));
    }
    input
}

fn crear_mapa_input(input: &[u8], mut buscaminas: Buscaminas) -> Buscaminas {
    let mut indice_filas = 0;
    let mut indice_columnas = 0;

    for caracter in input{
        buscaminas.mapa[indice_filas][indice_columnas] = *caracter;
        if indice_columnas == buscaminas.cant_columnas -1 {
            indice_columnas = 0;
            indice_filas += 1;
        }
        else {
            indice_columnas += 1;
        }
    }
    buscaminas
}

pub fn jugar(mut input: String){
    println!("input en buscaminas.rs{:?}", input);

    let cant_filas = cant_filas(input.as_bytes());
    let cant_columnas = cant_columnas(input.len(), cant_filas);

    let mut buscaminas = Buscaminas::new(cant_filas, cant_columnas);

    input = quitar_enters(input, buscaminas.cant_filas, buscaminas.cant_columnas);
    buscaminas = crear_mapa_input(input.as_bytes(), buscaminas);

    //agregar_recuento_de_minas(mapa);

}