use crate::avanzado::dividir;

pub fn sumar(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(feature = "logging")]
pub fn loggear(mensaje: &str) {
    println!("LOG: {}", mensaje);
}


pub mod avanzado {

    pub fn dividir(a: f32, b: f32) -> f32 {
        a / b
    }
    
    
}
