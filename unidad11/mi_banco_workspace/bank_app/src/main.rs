use bank_lib::{Cliente, TipoCuenta};

fn main() {
    
    println!("Esto es una Funcion: {:?}", Cliente::metodo());
    
    let mut cliente = Cliente::nuevo("Juan", TipoCuenta::Ahorros);
    cliente.depositar(500.0);
    cliente.retirar(200.0);
    cliente.retirar(400.0); // Queremos ver saldo insuficiente
    
    cliente.mostrar();
        
}
