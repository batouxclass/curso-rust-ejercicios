use crate::banco::transaction;
use crate::banco::transaction::{depositar, Saldo};
use crate::decorador::notificador::{DecoradorEmail, Notificador, NotificadorBase};
use crate::encapsular::domain;
use crate::factory::zoo::crear_animal;
use crate::zoologico::zoo;
use crate::zoologico::zoo::{Animal, Gato, Perro};
use crate::strategy::pagos::{procesar_pago, EstrategiaPago, Paypal, TarjetaCredito};

mod encapsular;
mod banco;
mod zoologico;
mod strategy;
mod factory;
mod decorador;

fn main()  {
    
    let base = NotificadorBase;
    let con_email = DecoradorEmail { inner: &base};
    
    con_email.enviar("Has recibido un mensaje!");
    
    /*
    let mascota = crear_animal("perro");
    mascota.hablar();
    */   
    
    /*
    let tarjeta = TarjetaCredito;
    let paypal = Paypal;
    
    procesar_pago(&tarjeta, 1000.0);
    procesar_pago(&paypal, 2000.0);
*/
    
    /*
    // paso 4: vector de trait objets (DST)
    let mut zooligco: Vec<Box<dyn Animal>> = Vec::new();
    
    // paso 5: insertar animals diferentes
    zooligco.push(Box::new(Perro));
    zooligco.push(Box::new(Gato));
    
    for animal in zooligco {
        animal.hablar();
    }
    */
    
    /*
    let cuenta = transaction::AccountNumer::new("12345678").unwrap();
    let saldo_inicial: Saldo = 1000.0;
    
    println!("Cuenta creada: {}", cuenta.get_number());
    println!("Saldo inicial: {}", saldo_inicial);
    
    let saldo_final = depositar(saldo_inicial, 500.0);
    println!("Saldo final: {}", saldo_final);
    */
    
    /*
    let user_id = domain::UserId::new("clase_en_rust").unwrap();    
    println!("El id es {}", user_id.get_id());    
    // user_id.0
    */  
    
}