use crate::resumen::Resumen;

pub fn notificacion(contenido: &impl Resumen) {
    println!("Noticia de última hora! {} ", contenido.resumir());
}