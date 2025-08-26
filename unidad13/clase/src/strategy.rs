
pub mod pagos {
    
    pub trait EstrategiaPago {
        fn pagar(&self, monto: f64);
    }
    
    pub struct TarjetaCredito;
    pub struct Paypal;
    
    impl EstrategiaPago for TarjetaCredito {
        fn pagar(&self, monto: f64) {
            println!("Pagando {} USD con tarjeta de crétido", monto);
        }
    }
    
    impl EstrategiaPago for Paypal {
        fn pagar(&self, monto: f64) {
            println!("Pagando {} USD con Paypal", monto);
        }
    }
    
    pub fn procesar_pago(metodo: &dyn EstrategiaPago, monto: f64) {
        metodo.pagar(monto);
    }    
    
}