
#[derive(Debug)]
pub enum TipoCuenta {
    Ahorros,
    Corriente,
}

#[derive(Debug)]
pub struct Cliente {
    pub nombre: String,
    pub saldo: f64,
    pub cuenta: TipoCuenta,
}

impl Cliente {
    pub fn nuevo(nombre: &str, cuenta: TipoCuenta) -> Self {
        Cliente {
            nombre: nombre.to_string(),
            saldo: 0.0,
            cuenta
        }
    }

    pub fn metodo() {
        println!("Soy un método!!");
    }

    pub fn depositar(&mut self, monto: f64) {
        if monto > 0.0 {
            self.saldo += monto;
            println!("-> Depósito exitoso de ${}", monto);
        } else {
            println!("El monto debe ser positivo.");
        }
    }

    pub fn retirar(&mut self, monto: f64) {
        if monto > self.saldo {
            println!("-> Fondos insuficientes. Saldo actual ${}", self.saldo);
        } else if monto <= 0.0 {
            println!("El monto debe ser positivo.");
        } else {
            self.saldo -= monto;
            println!("Retiro exitoso de ${}", monto);
        }
    }

    pub fn mostrar(&self) {
        println!("Cliente: {}", self.nombre);
        println!("Tipo de cuenta: {:?}", self.cuenta);
        println!("Saldo: ${}", self.saldo);
    }
}