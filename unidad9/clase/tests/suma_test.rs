pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

#[cfg(test)]
mod test {
    use crate::add;

    #[test]
    fn it_works() {
        let resultado = add(1.0, 2.0);
        assert_eq!(resultado, 3.0);
    }
}