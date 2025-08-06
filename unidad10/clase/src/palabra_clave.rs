use std::{env, process};
use clap::Parser;

/// Un programa que revela un secreto si se le proporciona la palabra clave secreta.
#[derive(Parser, Debug)]
#[command(about, version, author, about, long_about = None)]
struct PalabraClave {
    /// La palabra clave para acceder al secreto.
    palabra: String,
}
/*
fn main() {
    let args =  crate::PalabraClave::parse();

    if args.palabra == "secreto" {
        let variable_entorno = env::var("MI_API_KEY2");

        match variable_entorno {
            Ok(valor) => println!("{}", valor),
            Err(_) => {
                println!("La variable MI_API_KEY no está definida.");
                process::exit(1);
            }
        }
    } else {
        println!("Frase incorrecta: {}", args.palabra );
    }
}
*/
