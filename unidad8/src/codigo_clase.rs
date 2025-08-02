pub fn mayor<T: std::cmp::PartialOrd>(lista: &[T]) -> &T {
    let mut mayor = &lista[0];
    
    for numero in lista {
        if numero > mayor {
            mayor = numero;
        }
    }
    
    mayor
}