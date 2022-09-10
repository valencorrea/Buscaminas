//! Service principal, ya que es el que contiene la logica mas importante del
//! buscaminas. Se encuentra modularizado en las distintas funcionalidades requeridas para
//! lograr develar las minas.

use crate::calculadora_service::calcular_col_limite_der;
use crate::calculadora_service::calcular_col_limite_izq;
use crate::calculadora_service::calcular_fil_limite_inf;
use crate::calculadora_service::calcular_fil_limite_sup;
use crate::calculadora_service::cant_columnas;
use crate::calculadora_service::cant_filas;
use std::string::ToString;

pub const BOMBA_STRING: &str = "*";
pub const SIN_BOMBAS_STRING: &str = ".";
pub const ENTER_STRING: &str = "\n";

pub const BOMBA_U8: u8 = 42;
pub const NUM_CERO_U8: u8 = 48;

#[derive(Debug)]
/// Struct de Buscaminas
/// Contiene todas las estructuras necesarias para facilitar la implementación
struct Buscaminas {
    mapa: Vec<Vec<u8>>,
    cant_filas: usize,
    cant_columnas: usize,
}

/// Constructor del Buscaminas. Recibe los atributos necesarios para crearlo.
impl Buscaminas {
    pub fn new(cant_filas: usize, cant_columnas: usize) -> Self {
        Buscaminas {
            mapa: vec![vec![0; cant_columnas]; cant_filas],
            cant_filas,
            cant_columnas,
        }
    }
}

/// Función principal. Dentro se encuentran modularizados los distintos llamados
/// segun las funcionalidades requeridas para completar el descubrimiento de minas.
/// Recibe el mapa input y retorna el mapa con el conteo de las minas adyacentes, ambos
/// en formato String.
///
/// *Ejemplo de input:*
/// ```
/// "..*\n...\n*..\n"
/// ```
/// *Ejemplo de output:*
/// ```
/// ".1*\n121\n*1.\n"
/// ```
pub fn descubrir_minas(mut input: String) -> String {
    let cant_filas = cant_filas(input.as_bytes());
    let cant_columnas = cant_columnas(input.len(), cant_filas);

    let mut buscaminas = Buscaminas::new(cant_filas, cant_columnas);

    input = quitar_enters(input, buscaminas.cant_filas, buscaminas.cant_columnas);
    buscaminas = crear_mapa_input(input.as_bytes(), buscaminas);
    buscaminas = agregar_recuento_de_minas(buscaminas);
    pasar_mapa_a_string(buscaminas)
}

/// Funcion que suprime los enters de un string y lo retorna.
fn quitar_enters(mut input: String, cant_filas: usize, cant_columnas: usize) -> String {
    for indice in 0..cant_filas {
        input.remove(cant_columnas * (indice + 1));
    }
    input
}

/// Funcion que a partir de una tira de bytes, ubica cada uno de ellos en su respectiva
/// posicion correspondiente al mapa de Buscaminas.
/// Retorna un Buscaminas.
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

/// Función que dado un Buscaminas, agrega a cada una de las posiciones de su respectivo mapa
/// la cantidad correcta de minas adyacentes a dicha posicion, representado como u8.
/// En caso de no poseer minas adyacentes deja la posicion vacia.
/// Retorna el Buscaminas.
fn agregar_recuento_de_minas(mut buscaminas: Buscaminas) -> Buscaminas {
    let mut mapa_con_minas =
        vec![vec![NUM_CERO_U8; buscaminas.cant_columnas]; buscaminas.cant_filas];

    let mut indice_filas = 0;
    let mut indice_columnas = 0;

    while indice_filas < buscaminas.cant_filas {
        while indice_columnas < buscaminas.cant_columnas {
            if buscaminas.mapa[indice_filas][indice_columnas] != BOMBA_U8 {
                mapa_con_minas[indice_filas][indice_columnas] =
                    calcular_minas_adyacentes(indice_filas, indice_columnas, &buscaminas) as u8
                        + NUM_CERO_U8;
            } else {
                mapa_con_minas[indice_filas][indice_columnas] = BOMBA_U8;
            }
            indice_columnas += 1;
        }
        indice_filas += 1;
        indice_columnas = 0;
    }

    buscaminas.mapa = mapa_con_minas;
    buscaminas
}

/// Funcion que dada una posicion determinada en el mapa de Buscaminas, calcula la cantidad
/// de minas adyacentes a esa posicion y la retorna.
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

/// Funcion que dado un Buscaminas, obtiene y pasa su mapa de u8 a String y lo retorna.
fn pasar_mapa_a_string(buscaminas: Buscaminas) -> String {
    let mut output = String::new();
    for fila in 0..buscaminas.cant_filas {
        for columna in 0..buscaminas.cant_columnas {
            if buscaminas.mapa[fila][columna] == BOMBA_U8 {
                output.push_str(BOMBA_STRING);
            } else if buscaminas.mapa[fila][columna] != NUM_CERO_U8 {
                let cant_bombas = buscaminas.mapa[fila][columna] as i32 - NUM_CERO_U8 as i32;
                output.push_str(&*cant_bombas.to_string());
            } else {
                output.push_str(SIN_BOMBAS_STRING);
            }
        }
        output.push_str(ENTER_STRING);
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIN_BOMBA_U8: u8 = 46;

    #[test]
    fn paso_mapa_de_u8_a_string_ok() {
        let mut buscaminas_u8 = Buscaminas::new(2, 2);
        buscaminas_u8.mapa =
            vec![vec![BOMBA_U8; buscaminas_u8.cant_columnas]; buscaminas_u8.cant_filas];
        assert_eq!(pasar_mapa_a_string(buscaminas_u8), "**\n**\n");
    }

    #[test]
    fn calculo_minas_adyacentes_a_posicion_central() {
        let mut buscaminas_u8 = Buscaminas::new(3, 3);
        buscaminas_u8.mapa =
            vec![vec![SIN_BOMBA_U8; buscaminas_u8.cant_columnas]; buscaminas_u8.cant_filas];
        buscaminas_u8.mapa[0][0] = BOMBA_U8;
        buscaminas_u8.mapa[0][1] = BOMBA_U8;
        buscaminas_u8.mapa[1][2] = BOMBA_U8;
        assert_eq!(calcular_minas_adyacentes(1, 1, &buscaminas_u8), 3);
    }

    #[test]
    fn calculo_minas_adyacentes_a_posicion_en_esquina() {
        let mut buscaminas_u8 = Buscaminas::new(3, 3);
        buscaminas_u8.mapa =
            vec![vec![SIN_BOMBA_U8; buscaminas_u8.cant_columnas]; buscaminas_u8.cant_filas];
        buscaminas_u8.mapa[0][0] = BOMBA_U8; // No es adyacente
        buscaminas_u8.mapa[1][1] = BOMBA_U8;
        buscaminas_u8.mapa[1][2] = BOMBA_U8;
        assert_eq!(calcular_minas_adyacentes(2, 2, &buscaminas_u8), 2);
    }

    #[test]
    fn agrego_recuento_de_bombas_matriz_2x3_2_bombas() {
        let mut buscaminas_sin_minas = Buscaminas::new(2, 3);
        buscaminas_sin_minas.mapa = vec![
            vec![SIN_BOMBA_U8; buscaminas_sin_minas.cant_columnas];
            buscaminas_sin_minas.cant_filas
        ];
        buscaminas_sin_minas.mapa[0][0] = BOMBA_U8;
        buscaminas_sin_minas.mapa[1][2] = BOMBA_U8;

        let mut buscaminas_con_minas = Buscaminas::new(2, 3);
        buscaminas_con_minas.mapa = vec![
            vec![NUM_CERO_U8; buscaminas_sin_minas.cant_columnas];
            buscaminas_sin_minas.cant_filas
        ];
        buscaminas_con_minas.mapa[0][0] = BOMBA_U8;
        buscaminas_con_minas.mapa[0][1] = NUM_CERO_U8 + 2;
        buscaminas_con_minas.mapa[0][2] = NUM_CERO_U8 + 1;
        buscaminas_con_minas.mapa[1][0] = NUM_CERO_U8 + 1;
        buscaminas_con_minas.mapa[1][1] = NUM_CERO_U8 + 2;
        buscaminas_con_minas.mapa[1][2] = BOMBA_U8;

        // Se puede chequear corriendo corriendo "otro_mapa_input.txt" ya que inclui este caso ahi.
        assert_eq!(
            agregar_recuento_de_minas(buscaminas_sin_minas).mapa,
            buscaminas_con_minas.mapa
        );
    }
}
