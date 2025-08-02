struct Rectangulo {
    ancho: i32,
    largo: i32
}

impl Rectangulo {
    fn puede_sostener(&self, otro: &Rectangulo) -> bool {
        self.ancho == otro.ancho &&  self.largo == otro.largo
    }
}

#[cfg(test)]
mod tests {
    use crate::Rectangulo;

    #[test]
    fn mas_larg_puede_sostener_mas_pequenio() {
        let mas_largo = Rectangulo {
            ancho: 10,
            largo: 7
        };
        
        let mas_pequenio = Rectangulo {
            ancho: 10,
            largo: 7
        };
        
        assert!(mas_largo.puede_sostener(&mas_pequenio))
    }
    
}