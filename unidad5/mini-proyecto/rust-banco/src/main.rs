use nucleo_banco::cliente::Cliente;
use nucleo_banco::cuenta::TipoCuenta;

fn main() {
    let mut juan = Cliente::nueva("Juan", TipoCuenta::Ahorros);
    
    println!("Iniciando transacciones");
    
    juan.mostrar();
    juan.depositar(1200000.0);
    juan.mostrar();
    juan.retirar(200000.0);
    juan.mostrar();

    println!("Finalizando transacciones");
}
