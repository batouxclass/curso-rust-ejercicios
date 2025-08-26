pub mod zoo{
    // paso # 1
    pub trait Animal {
        fn hablar(&self);
    }
    
    // paso 2
    
    pub struct Perro;
    pub struct Gato;
    
    // paso 3
    impl Animal for Perro {
        fn hablar(&self) {
            println!("El perro dice: ¡Guau guau!");
        }        
    }
    
    impl Animal for Gato {
        fn hablar(&self)  {
            println!("El gato: ¡Miau!");
        }
    }
    
}