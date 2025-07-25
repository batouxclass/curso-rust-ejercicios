use std::fs::File;
use std::io;
use std::io::Read;

fn gestionar_archivo() -> Result<String, io::Error> {
    let saludo = File::open("saludo.txt");
    
    let mut saludo_archivo = match saludo {
        Ok(archivo) => archivo,
        Err(error) => return Err(error),
    };
    
    let mut contenido = String::new();
    
    match saludo_archivo.read_to_string(&mut contenido) {
        Ok(_) => Ok(contenido),
        Err(error) => return Err(error),
    }
}

fn main() {
    let resultado = gestionar_archivo();
    
    match  resultado { 
        Ok(resultado) => println!("correcto: {}", resultado),
        Err(error) => println!("Por favor crear el archivo: {}", error),
    }
}