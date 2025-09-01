use std::io;
use std::time::Duration;
use crossterm::{cursor, event, execute, terminal};
use crossterm::cursor::MoveTo;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType};
use crate::Juego::{Direccion, EstadoJuego};
use crate::render::{CuadriculaRenderer, Render};

mod render;
mod Juego;

fn main() -> io::Result<()> {
    terminal::enable_raw_mode()?;

    let mut renderer = CuadriculaRenderer::new( (30, 20))?;    
    execute!(renderer.stdout, cursor::Hide, Clear(ClearType::All))?;
    let mut estado_juego = EstadoJuego::new(renderer.cuadricula);
    
    let (ancho, alto) = renderer.cuadricula;
    execute!(
        renderer.stdout,
        cursor::MoveTo(ancho / 2 - 5, alto / 2),
        Print("INICIO DEL JUEGO")
    )?;

    execute!(
        renderer.stdout,
        cursor::MoveTo(ancho / 2 - 12, alto / 3),
        Print("Presiona cualquier tecla para empezar...")
    )?;

    // Espera a una tecla antes de continuar
    loop {
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key_event) = event::read()? {
                if key_event.kind == KeyEventKind::Press {
                    break;
                }
            }
        }
    }

    // Limpia la pantalla después de la pantalla de inicio
    execute!(renderer.stdout, terminal::Clear(terminal::ClearType::All))?;
            
    // Bucle principal del juego
    loop {
        // 1. Manejamos los controles del juego con las teclas:
        // W: Arriba
        // S: Abajo
        // A: Izquierda
        // D: Derecha
        // ⬆️: Arriba
        // ⬇️: Abajo
        // ⬅️: Izquierda
        // ➡️: Derecha
        // 
        
        if event::poll(Duration::from_millis(0))? {
            // solo reacciona si hay un evento de una tecla
            if let Event::Key(evento_tecla) = event::read()? {
                if evento_tecla.kind == KeyEventKind::Press {
                    let nueva_direccion = match evento_tecla.code {
                        KeyCode::Char('w') | KeyCode::Up => Some(Direccion::Arriba),
                        KeyCode::Char('s') | KeyCode::Down => Some(Direccion::Abajo),
                        KeyCode::Char('a') | KeyCode::Left => Some(Direccion::Izquierda),
                        KeyCode::Char('d') | KeyCode::Right => Some(Direccion::Derecha),
                        KeyCode::Char('q') | KeyCode::Esc => break, // sale del juego.
                        _ => None,
                    };

                    if let Some(direccion) = nueva_direccion {
                        estado_juego.cambiar_direccion(direccion);
                    }
                }
            }
        }
        
        // Actualizamos el estado del juego
        estado_juego.actualizar();
        
        if estado_juego.es_fin_juego() {
            break; 
        }
        
        // Renderizar el juego usando el trait
        renderer.render(&estado_juego)?;
        
        // Controlar la velocidad del juego
        std::thread::sleep(std::time::Duration::from_millis(900));
    }

    execute!(
        renderer.stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(ancho / 2 - 5, alto / 2),
        Print("FIN DEL JUEGO!")
    )?;

    execute!(
        renderer.stdout,
        cursor::MoveTo(ancho / 2 - 8, alto / 2 + 1),
        Print(format!("Puntuación final: {}", estado_juego.puntaje()))
    )?;

    // Mueve el curso al inicio para la siguiente actualización
    execute!(renderer.stdout, MoveTo(0, alto + 4))?;
    
    // Nos aseguramos de retornar la terminal a su estado original
    execute!(renderer.stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
