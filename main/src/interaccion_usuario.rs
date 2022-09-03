//use std::io;

pub fn dar_bienvenida(){
    println!("\nBIENVENIDOS AL BUSCAMINAS!!!\n");
    println!("Con el siguiente programa vas a poder etcetc\n");
    println!("Leyendo mapa...\n");
}

pub fn mostrar_mapa(instancia:&String, mapa: &String){
    println!("Mostrando mapa {instancia}:\n\n{mapa}");
}


let mut linea = String::new();
file.read_line(&mut linea).unwrap();

println!("{}", linea);

let arr: Vec<Vec<f64>> = file.lines()
.map(|l| l.unwrap().as_bytes()/*.split(char::is_whitespace)*/
.map(|l| l.parse().unwrap())
.collect())
.collect();

println!("{:?}", arr);