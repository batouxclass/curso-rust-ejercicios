pub mod transaction {
    
    // paso 1: alias de tipo
    pub type Saldo = f64;
    
    // paso2: newtype para encapsultar el numero de cuenta
    #[derive(Debug)]
    pub struct AccountNumer(String);
    
    impl AccountNumer {
        // paso 3: crear el constructor
        pub fn new (numero: &str) -> Option<Self> {
            if (!numero.is_empty()) {
                Some(AccountNumer(numero.to_string()))
            } else {
                None
            }            
        }
        
        // paso 4: meotod get number
        pub fn get_number(&self) -> &str {
            &self.0
        }
    }
    
    pub fn depositar(saldo: Saldo, monto: f64) -> Saldo {
        saldo + monto
    }
    
}