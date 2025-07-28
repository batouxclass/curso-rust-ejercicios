use std::fmt;

#[derive(Debug)]
pub enum TipoCuenta {
    Ahorros,
    Corriente,
}
#[derive(Debug)]
pub struct Cuenta {
   pub tipo: TipoCuenta,
   pub saldo: f64,
}

#[derive(Debug)]
pub struct Cliente {
    pub nombre: String,
    pub cuenta: Cuenta
}

#[derive(Debug)]
pub enum BancoError {
    MontoNegativo(f64),
    FondosInsuficientes(f64)
}

impl fmt::Display for BancoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BancoError::MontoNegativo(valor) => write!(
                f, "El monto no puede ser negativo: {valor}"),
            BancoError::FondosInsuficientes(valor) => write!(
                f, "Fondos insuficientes para esta operacion: {valor}"),
        }
    }
}

impl Cuenta {
    fn desositar(&mut self, monto: f64) -> Result<(), BancoError> {
        if monto < 0.0 {
            return Err(BancoError::MontoNegativo(monto))
        }
        self.saldo += monto;
        Ok(())
    }
    
    fn retirar(&mut self, monto: f64) -> Result<(), BancoError> {
        if monto < 0.0 {
            return Err(BancoError::MontoNegativo(monto))
        }
        if monto > self.saldo {
            return Err(BancoError::FondosInsuficientes(monto))
        }
        self.saldo -= monto;
        Ok(())
    }
}

impl Cliente {
    pub fn operar(&mut self) -> Result<(), BancoError> {
        self.cuenta.desositar(1000.0)?;
        self.cuenta.retirar(250.0)?;
        self.cuenta.retirar(800.0)?;
        Ok(())
    }
}