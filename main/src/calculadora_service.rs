const ENTER_U8 : u8 = 10;

pub fn cant_filas(bytes: &[u8]) -> usize { // PRE: TIENE ENTERS AL FINAL DE CADA FILA
    let mut cant_filas = 0;

    for caracter in bytes {
        if *caracter == ENTER_U8 {
            cant_filas += 1;
        }
    }
    cant_filas
}

pub fn cant_columnas(cant_bytes_matriz: usize, filas: usize) -> usize { // PRE: TIENE ENTERS AL FINAL DE CADA FILA
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