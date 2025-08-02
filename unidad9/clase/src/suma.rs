pub fn add2(a: f64, b: f64) -> f64 {
    a + b
}

#[cfg(test)]
mod test {
    use crate::suma::add2;

    #[test]
    fn it_works2() {
        let resultado = add2(1.0, 2.0);
        assert_eq!(resultado, 3.0);
    }
}