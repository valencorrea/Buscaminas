use std::ops::Add;

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

fn quitar_enters(mut input: String, cant_filas: usize, cant_columnas: usize) -> String {
    for indice in 0..cant_filas {
        input.remove(cant_columnas * (indice + 1));
    }
    input
}

fn crear_mapa_input(input: &[u8], mut buscaminas: Buscaminas) -> Buscaminas {
    let mut indice_filas = 0;
    let mut indice_columnas = 0;

    for caracter in input {
        buscaminas.mapa[indice_filas][indice_columnas] = *caracter;
        if indice_columnas == buscaminas.cant_columnas - 1 {
            indice_columnas = 0;
            indice_filas += 1;
        } else {
            indice_columnas += 1;
        }
    }
    buscaminas
}

fn calcular_bombas_adyacentes(
    indice_filas: usize,
    indice_columnas: usize,
    buscaminas: &Buscaminas,
) -> i32 {
    // llevar todas a difs funciones
    let mut cant_bombas = 0;
    let mut fil_limite_sup = if indice_filas == 0 { 0 } else { indice_filas - 1 };
    let mut fil_limite_inf = if (indice_filas + 2) >= buscaminas.cant_filas { buscaminas.cant_filas } else { indice_filas + 2 };
    let mut col_limite_izq = if indice_columnas == 0 { 0 } else { indice_columnas - 1 };
    let mut col_limite_der = if (indice_columnas + 2) >= buscaminas.cant_columnas { buscaminas.cant_columnas } else { indice_columnas + 2 };

    for fil in fil_limite_sup..fil_limite_inf {
        for col in col_limite_izq..col_limite_der {
            if buscaminas.mapa[fil][col] == 42 {
                cant_bombas += 1;
            }
        }
    }
    cant_bombas
}

fn agregar_recuento_de_minas(mut buscaminas: Buscaminas) -> Buscaminas {
    println!("{:?}", buscaminas.mapa);

    let mut mapa_con_bombas = vec![vec![48; buscaminas.cant_columnas]; buscaminas.cant_filas];

    for indice_filas in 0..buscaminas.cant_filas {
        for indice_columnas in 0..buscaminas.cant_columnas {
            if buscaminas.mapa[indice_filas][indice_columnas] != 42 {
                mapa_con_bombas[indice_filas][indice_columnas] = calcular_bombas_adyacentes(indice_filas, indice_columnas, &buscaminas) as u8 + 48;
                /*if buscaminas.mapa[indice_filas][indice_columnas] == 48 {
                    mapa_con_bombas[indice_filas][indice_columnas] = 46;
                }*/
            } else {
                mapa_con_bombas[indice_filas][indice_columnas] = 42;
            }
        }
    }
    println!("{:?}", buscaminas.mapa);

    buscaminas.mapa = mapa_con_bombas;
    buscaminas
}


fn pasar_mapa_a_string(buscaminas: Buscaminas) -> String {
    let mut output = String::new();

    for fila in 0..buscaminas.cant_filas {
        for columna in 0..buscaminas.cant_columnas{
            if buscaminas.mapa[fila][columna] == 42 {
                output.push_str("*");
            }
            else if buscaminas.mapa[fila][columna] != 48 {
                let mut cant_bombas = buscaminas.mapa[fila][columna] as i32 - 48;
                output.push_str(&*cant_bombas.to_string());
            }
            else {
                output.push_str(".");
            }
        }
        output.push_str("\n");
    }
    output.remove(output.len()-1);// saco el ultimo \n
    output
}


pub fn jugar(mut input: String) {
    let cant_filas = cant_filas(input.as_bytes());
    let cant_columnas = cant_columnas(input.len(), cant_filas);

    let mut buscaminas = Buscaminas::new(cant_filas, cant_columnas);

    input = quitar_enters(input, buscaminas.cant_filas, buscaminas.cant_columnas);
    buscaminas = crear_mapa_input(input.as_bytes(), buscaminas);

    buscaminas = agregar_recuento_de_minas(buscaminas);
    let mut output = pasar_mapa_a_string(buscaminas);

    println!("{:?}", output);

}

    /*
    [[1, *, 3, *, 1],
    [1, 3, *, 3, 1],
    [., 2, 2, 2, .],
    [., 1, 1, 1, .]]
    */
