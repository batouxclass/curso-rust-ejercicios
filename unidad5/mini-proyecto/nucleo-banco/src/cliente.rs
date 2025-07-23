use crate::auditoria::log;
use crate::cuenta::TipoCuenta;
pub struct Cliente {
    nombre: String,
    saldo: f64,
    cuenta: TipoCuenta,
}

impl Cliente {
    pub fn nueva(nombre: &str, cuenta: TipoCuenta) -> Self {
        Cliente {
            nombre: nombre.to_string(),
            saldo: 0.0,
            cuenta,
        }
    }

    pub fn depositar(&mut self, monto: f64) {
        self.saldo += monto;
        let mensaje = format!("{} depositó {}", self.nombre,  monto);
        #[cfg(feature = "auditoria")]
        log(&mensaje);
    }

    pub fn retirar(&mut self, monto: f64) {
        let permitido = match self.cuenta {
            TipoCuenta::Ahorros => self.saldo >= monto,
            TipoCuenta::Corriente => self.saldo - monto >= -500000.0,
        };

        if permitido {
            self.saldo -= monto;
            let mensaje = format!("{} retiró {}",  self.nombre, monto);
            #[cfg(feature = "auditoria")]
            log(&mensaje);
        } else {
            log("no tiene fondos suficientes");
        }
    }

    pub fn mostrar(&self) {
        #[cfg(feature = "auditoria")]
        log(&format!("Nombre: {}", self.nombre ));
        #[cfg(feature = "auditoria")]
        log( &format!("Saldo: {}", self.saldo));
        #[cfg(feature = "auditoria")]
        log( &format!("Cuenta: {:?}", self.cuenta));
    }

}