
pub mod zoo {

    pub trait Animal {
        fn hablar(&self);
    }
    
    pub struct Perro;
    pub struct Gato;
    
    impl Animal for Perro {
        fn hablar(&self) {
            println!("Guau")
        }
    }
    
    impl Animal for Gato {
        fn hablar(&self) { 
            println!("Miau")
        }
    }
    
    pub fn crear_animal(tipo: &str) -> Box<dyn Animal> {
        match tipo {
            "perro" => Box::new(Perro),
            "gato" => Box::new(Gato),
            _ => panic!("Tipo de animal desconocido"),
        }
    }
}
