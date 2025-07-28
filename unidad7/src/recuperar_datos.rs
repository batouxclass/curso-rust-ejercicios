pub fn recuperar_datos() -> Result<String, String> {
    let data = std::fs::read_to_string("archivo.txt");

    match data {
        Ok(contenido) => Ok(contenido),
        Err(error) => Err("no se logro abrir el archivo".to_string()),
    }
}
