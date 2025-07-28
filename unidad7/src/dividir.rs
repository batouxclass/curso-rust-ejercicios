pub fn dividir (a:f32, b:f32) -> Result<String, String> {
    if (b == 0.0) {
        Err("Dividiendo por cero".into())
    } else {
        let result = a / b;
        Ok(result.to_string())
    }
}