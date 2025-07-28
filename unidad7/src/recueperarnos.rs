use std::fs::File;
use std::io::ErrorKind;

pub fn recuperarnos() -> File {
    let saludo = File::open("saludo.txt");

    let saludo_archivo = match saludo {
        Ok(saludo) => saludo,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("saludo.txt") {
                Ok(saludo) => saludo,
                Err(e) => panic!("Problema creando el archivo {}", error),
            }
            _ => panic!("Problema creando el archivo"),
        }
    };
    
    saludo_archivo
}