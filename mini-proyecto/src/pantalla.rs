use crate::Juego::EstadoJuego;

pub trait Render {
    fn render(&self, juego: &EstadoJuego);
}