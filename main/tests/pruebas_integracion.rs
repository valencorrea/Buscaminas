//use crate::main::src::buscaminas_service;

#[path = "../src/buscaminas_service.rs"]
mod buscaminas_service;

#[path = "../src/calculadora_service.rs"]
mod calculadora_service;


#[test] // <- no la corre
fn descubrir_minas_input_3x3_1_mina() {
    let string_input = String::from("...\n...\n..*\n");
    let string_output = String::from("...\n.11\n.1*\n");
    assert_eq!(
        buscaminas_service::descubrir_minas(string_input),
        string_output
    );
}
