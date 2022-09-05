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

fn calcular_bombas_adyacentes(indice_filas: usize, indice_columnas: usize, buscaminas: &Buscaminas) -> i32 {
    let mut cant_bombas = 0;
    let mut fil_limite_sup = if indice_filas == 0 { 0 } else { indice_filas - 1};
    let mut fil_limite_inf = if (indice_filas + 2) >= buscaminas.cant_filas { buscaminas.cant_filas } else { indice_filas + 2 };
    let mut col_limite_izq = if indice_columnas == 0 { 0 } else { indice_columnas - 1 };
    let mut col_limite_der = if (indice_columnas + 2) >= buscaminas.cant_columnas { buscaminas.cant_columnas } else { indice_columnas + 2 };

    println!("fil sup {}\n fil inf {}\n col izq{}\n col der{}\n", fil_limite_sup, fil_limite_inf, col_limite_izq, col_limite_der);

    for fil in fil_limite_sup..fil_limite_inf{
        for col in col_limite_izq..col_limite_der{
            if (buscaminas.mapa[fil][col] == 42) /*&& !(fil == indice_filas) && (col == indice_columnas)*/ {
                cant_bombas +=1;
            }
        }
    }
    println!("cant de bombas para [{}][{}] = {}\n\n", indice_filas, indice_columnas, cant_bombas);
    cant_bombas
}

fn agregar_recuento_de_minas(mut buscaminas: Buscaminas) -> Vec<Vec<i32>> {
    println!("{:?}", buscaminas.mapa);

    let mut mapa_con_bombas = vec![vec![0; buscaminas.cant_columnas]; buscaminas.cant_filas];

    for indice_filas in 0..buscaminas.cant_filas {
        for indice_columnas in 0..buscaminas.cant_columnas {
            mapa_con_bombas[indice_filas][indice_columnas] = calcular_bombas_adyacentes(indice_filas, indice_columnas, &buscaminas);
        }
    }
    mapa_con_bombas
}

pub fn jugar(mut input: String){

    let cant_filas = cant_filas(input.as_bytes());
    let cant_columnas = cant_columnas(input.len(), cant_filas);

    let mut buscaminas = Buscaminas::new(cant_filas, cant_columnas);

    input = quitar_enters(input, buscaminas.cant_filas, buscaminas.cant_columnas);
    buscaminas = crear_mapa_input(input.as_bytes(), buscaminas);

    let output = agregar_recuento_de_minas(buscaminas);
    println!("{:?}", output);
/*
    [[1, 2, 3, 2, 1],
    [1, 3, 4, 3, 1],
    [0, 2, 2, 2, 0],
    [0, 1, 1, 1, 0]]

    1*3*1
    13*31
    -2*2-
    -111-*/

}