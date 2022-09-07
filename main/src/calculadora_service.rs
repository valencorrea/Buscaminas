//! Este modulo tiene como objetivo centralizar las operaciones y calculos
//! realizados en base a las filas y columnas de la matriz del Buscaminas.
//!
//! Todas las funciones de calculo de limites calculan el respectivo rango
//! sobre el cual se podra iterar alrededor de una determinada posicion

const ENTER_U8: u8 = 10;

/// Getter de la cantidad de filas del Buscaminas.
/// La tira de bytes recibida contiene un enter al finalizar cada fila.
/// Ejemplo para una matriz 2x2 de bombas:
/// ```
/// [48, 48, 10, 48, 48, 10]
/// ```
pub fn cant_filas(bytes: &[u8]) -> usize {
    let mut cant_filas = 0;

    for caracter in bytes {
        if *caracter == ENTER_U8 {
            cant_filas += 1;
        }
    }
    cant_filas
}

/// Getter de la cantidad de columnas del Buscaminas.
/// Al igual que el getter de las filas, también recibe la tira de bytes con
/// los enters incluidos, razon por la cual al finalizar se le resta 1.
/// Ejemplo de una matriz 2x3 de bombas:
/// ```
/// [48, 48, 48, 10, 48, 48, 48, 10]
/// ```
/// (8 bytes matriz / 2 filas) - enter = 3
pub fn cant_columnas(cant_bytes_matriz: usize, filas: usize) -> usize {
    (cant_bytes_matriz / filas) - 1
}

pub fn calcular_fil_limite_sup(indice_filas: usize) -> usize {
    if indice_filas == 0 {
        0
    } else {
        indice_filas - 1
    }
}

pub fn calcular_fil_limite_inf(indice_filas: usize, cant_filas: usize) -> usize {
    if (indice_filas + 2) >= cant_filas {
        cant_filas
    } else {
        indice_filas + 2
    }
}

pub fn calcular_col_limite_izq(indice_columnas: usize) -> usize {
    if indice_columnas == 0 {
        0
    } else {
        indice_columnas - 1
    }
}

pub fn calcular_col_limite_der(indice_columnas: usize, cant_columnas: usize) -> usize {
    if (indice_columnas + 2) >= cant_columnas {
        cant_columnas
    } else {
        indice_columnas + 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matriz_nxm_calculo_cant_filas_retorna_n() {
        assert_eq!(cant_filas(&[42, 42, 42, ENTER_U8, 42, 42, 42, ENTER_U8]), 2);
    }

    #[test]
    fn matriz_nxm_calculo_cant_columnas_retorna_m() {
        assert_eq!(cant_columnas(8, 2), 3);
    }
}
