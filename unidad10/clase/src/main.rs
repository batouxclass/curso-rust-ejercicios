use std::{fs, process};
use clap::Parser;

mod variables_ambiente;
mod clap_martes_args;
mod palabra_clave;

#[derive(Parser, Debug)]
#[command(about, version, author, about, long_about = None)]
struct Archivo {
    #[arg(short, long)]
    entrada: String,

    #[arg(short, long)]
    salida: String
}

//clase -e archivo_entrada.txt -s archivo_nuevo.txt

fn main() {

    let argumentos  = Archivo::parse();

    let entrada = argumentos.entrada;
    let contenido = fs::read_to_string(entrada);
    
    let contenido = match contenido {
        Ok(contenido) => contenido,
        Err(err) => {
            eprintln!("No se pudo leer el archivo de entrada: {}", err);
            process::exit(1);
        }        
    };

    let contenido_mayusculas = contenido.to_uppercase();

    let salida = argumentos.salida;
    
    fs::write(salida, contenido_mayusculas).unwrap();

}