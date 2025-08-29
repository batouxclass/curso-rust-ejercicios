use rand::Rng;

#[derive(Debug)]
pub enum Direccion {
    Arriba,
    Abajo,
    Izquierda,
    Derecha
}

#[derive(Debug)]
pub struct Posicion {
    x: u16,
    y: u16,
}

// tipo genericos para alimentos
pub struct Comida<T> {
    pub posicion: Posicion,
    pub valor: T,
}

pub struct EstadoJuego {
    serpiente: Vec<Posicion>,
    direccion: Direccion,
    comida: Comida<i32>,
    puntaje: i32,
    fin_juego: bool,
}

impl EstadoJuego {
    
    pub fn new() -> Self {
        let mut random = rand::thread_rng(); 
        
        random.gen_range((1..10));
        
        let serpiente =
            vec![Posicion {
                x: random.gen_range((1..10)),
                y: random.gen_range((1..10))}
            ];
        
        EstadoJuego {
            serpiente,
            direccion: Direccion::Derecha,
            comida: Comida {
                posicion: Posicion {
                    x: random.gen_range((1..10)),
                    y: random.gen_range((1..10)),
                },
                valor: 1,
            },
            puntaje: 0,
            fin_juego: false,
        }
        
    }
}
