use std::env;

#[warn(dead_code)]
fn variables() {
    let nombre_usuario = env::var("NOMBRE_USUARIO");

    match nombre_usuario {
        Ok(nombre) => println!("{}", nombre),
        Err(e) => println!("Error: NOMBRE_USUARIO no se pudo leer {}", e),
    }
}