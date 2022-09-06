use std::string::ToString;
use crate::calculadora_service::calcular_col_limite_der;
use crate::calculadora_service::calcular_col_limite_izq;
use crate::calculadora_service::calcular_fil_limite_inf;
use crate::calculadora_service::calcular_fil_limite_sup;
use crate::calculadora_service::cant_columnas;
use crate::calculadora_service::cant_filas;

const BOMBA_STRING : &str = "*";
const SIN_BOMBAS_STRING : &str = ".";
const ENTER_STRING : &str = "\n";
const BOMBA_U8 : u8 = 42;
const NUM_CERO_U8 : u8 = 48;

#[derive(Debug)] // ver si lo necesito
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

pub fn descubrir_minas(mut input: String) -> String {
    let cant_filas = cant_filas(input.as_bytes());
    let cant_columnas = cant_columnas(input.len(), cant_filas);

    let mut buscaminas = Buscaminas::new(cant_filas, cant_columnas);

    input = quitar_enters(input, buscaminas.cant_filas, buscaminas.cant_columnas);
    buscaminas = crear_mapa_input(input.as_bytes(), buscaminas);
    buscaminas = agregar_recuento_de_minas(buscaminas);
    pasar_mapa_a_string(buscaminas)
}

fn quitar_enters(mut input: String, cant_filas: usize, cant_columnas: usize) -> String {
    for indice in 0..cant_filas {
        input.remove(cant_columnas * (indice + 1));
    }
    input
}

fn crear_mapa_input(input: &[u8], mut buscaminas: Buscaminas) -> Buscaminas {
    let mut fila = 0;
    let mut columna = 0;

    for caracter in input {
        buscaminas.mapa[fila][columna] = *caracter;
        if columna == buscaminas.cant_columnas - 1 {
            columna = 0;
            fila += 1;
        } else {
            columna += 1;
        }
    }
    buscaminas
}

fn agregar_recuento_de_minas(mut buscaminas: Buscaminas) -> Buscaminas {
    let mut mapa_con_minas = vec![vec![NUM_CERO_U8; buscaminas.cant_columnas]; buscaminas.cant_filas];

    for indice_filas in 0..buscaminas.cant_filas {
        for indice_columnas in 0..buscaminas.cant_columnas {
            if buscaminas.mapa[indice_filas][indice_columnas] != BOMBA_U8 {
                mapa_con_minas[indice_filas][indice_columnas] =
                    calcular_minas_adyacentes(indice_filas, indice_columnas, &buscaminas) as u8
                        + NUM_CERO_U8;
            } else {
                mapa_con_minas[indice_filas][indice_columnas] = BOMBA_U8;
            }
        }
    }
    buscaminas.mapa = mapa_con_minas;
    buscaminas
}

fn calcular_minas_adyacentes(
    indice_filas: usize,
    indice_columnas: usize,
    buscaminas: &Buscaminas,
) -> i32 {
    let mut cant_minas = 0;
    let fil_limite_sup = calcular_fil_limite_sup(indice_filas);
    let fil_limite_inf = calcular_fil_limite_inf(indice_filas, buscaminas.cant_filas);
    let col_limite_izq = calcular_col_limite_izq(indice_columnas);
    let col_limite_der = calcular_col_limite_der(indice_columnas, buscaminas.cant_columnas);

    for fila in fil_limite_sup..fil_limite_inf {
        for columna in col_limite_izq..col_limite_der {
            if buscaminas.mapa[fila][columna] == BOMBA_U8 {
                cant_minas += 1;
            }
        }
    }
    cant_minas
}

fn pasar_mapa_a_string(buscaminas: Buscaminas) -> String {
    let mut output = String::new();
    for fila in 0..buscaminas.cant_filas {
        for columna in 0..buscaminas.cant_columnas{
            if buscaminas.mapa[fila][columna] == BOMBA_U8 {
                output.push_str(BOMBA_STRING);
            }
            else if buscaminas.mapa[fila][columna] != NUM_CERO_U8 {
                let cant_bombas = buscaminas.mapa[fila][columna] as i32 - NUM_CERO_U8 as i32;
                output.push_str(&*cant_bombas.to_string());
            }
            else {
                output.push_str(SIN_BOMBAS_STRING);
            }
        }
        output.push_str(ENTER_STRING);
    }
    output
}
