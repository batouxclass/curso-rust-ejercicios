use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{ErrorKind, Read};
use crate::dividir::dividir;
use crate::mini_banco::{Cliente, Cuenta, TipoCuenta};
use crate::recuperar_datos::recuperar_datos;

mod recuperar_datos;
mod dividir;
mod recueperarnos;
mod mini_banco;

fn regestionar_archivo() -> Result<String, io::Error> {
    let mut saludo = File::open("saludo.txt")?;        
    let mut contenido = String::new();
    saludo.read_to_string(&mut contenido)?;    
    Ok(contenido)
}

fn main() {

    let cuenta = Cuenta {
        tipo: TipoCuenta::Ahorros,
        saldo: 0.0,
    };
    
    let mut cliente = Cliente {
        nombre: "Carlos".to_string(),
        cuenta: cuenta,
    };
    
    match cliente.operar() {
        Ok(_) => println!("Operacion exitosa"),
        Err(err) => println!("Error en operacion {}", err),
    }
    
    
    /*
    let result =  regestionar_archivo();
    
    match result {
        Ok(contenido) => println!("{}", contenido),
        Err(error) => println!("{}", error),
    }
    */
    
    /*
    let resultado = recueperarnos::recuperarnos().metadata().unwrap().len();
    println!("{:?}", resultado);
    */   
    
    /*
    let procesando = recuperar_datos();    
    println!("{:?}", procesando);

    let resultado = dividir(8.0, 0.0);
    
    match resultado {
        Ok(resultado_division) => println!("dividiendo {}", resultado_division ),
        Err(error) => println!("Error {}", error),
    }*/
        
}

