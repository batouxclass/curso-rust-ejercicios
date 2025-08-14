use std::cell::RefCell;
use std::rc::Rc;
use bank_lib::{Cliente, TipoCuenta};

fn main() {
    
    let cuenta_compartida =
        Rc::new(
            RefCell::new(
                Cliente::nuevo(
                    "Juan", 
                    TipoCuenta::AHORROS)));
    
    let juan = Rc::clone(&cuenta_compartida);
    let ana = Rc::clone(&cuenta_compartida);
    
    println!("contador de referencias: {}", Rc::strong_count(&cuenta_compartida));
    println!("-------------------------");
    // juan deposita
    juan.borrow_mut().depositar(500.0);
    juan.borrow().mostrar();
    println!("-------------------------");
    // ana deposita
    ana.borrow_mut().depositar(1500.0);
    ana.borrow().mostrar();
    println!("-------------------------");

    println!("contador de referencias: {}", Rc::strong_count(&cuenta_compartida));
}
