use std::collections::LinkedList;
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direccion {
    Arriba,
    Abajo,
    Izquierda,
    Derecha
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Posicion {
    pub x: u16,
    pub y: u16,
}

// tipo genericos para alimentos
pub struct Comida<T> {
    pub posicion: Posicion,
    pub valor: T,
}

pub struct EstadoJuego {
    serpiente: LinkedList<Posicion>,
    direccion: Direccion,
    comida: Comida<i32>,
    puntaje: i32,
    cuadricula: (u16, u16),
    fin_juego: bool,
}

impl EstadoJuego {
    
    pub fn new(cuadricula: (u16, u16)) -> Self {
        let mut random = rand::rng();
        let serpiente = LinkedList::from(
            [Posicion {
                    x: cuadricula.0 / 2,
                    y: cuadricula.1 / 2,
                }]
        );

        EstadoJuego {
            serpiente,
            direccion: Direccion::Derecha,
            comida: Comida {
                posicion: Posicion {
                    x: random.random_range((1..10)),
                    y: random.random_range((1..10)),
                },
                valor: 1,
            },
            cuadricula,
            puntaje: 0,
            fin_juego: false,
        }
    }

    fn emitir_comida(&mut self) {
        let mut random = rand::rng();
        self.comida.posicion = Posicion {
            x: random.random_range(1..self.cuadricula.0),
            y: random.random_range(1..self.cuadricula.1),
        }
    }

    pub fn actualizar(&mut self) {
        if self.fin_juego {
            return;
        }

        let avanzar = self.serpiente.front().unwrap();
        let nuevo_avanzar = match self.direccion {
            Direccion::Arriba => Posicion {
                x: avanzar.x,
                y: avanzar.y - 1,
            },
            Direccion::Abajo => Posicion {
                x: avanzar.x,
                y: avanzar.y + 1,
            },
            Direccion::Izquierda => Posicion {
                x: avanzar.x - 1,
                y: avanzar.y,
            },
            Direccion::Derecha => Posicion {
                x: avanzar.x + 1,
                y: avanzar.y,
            },
        };

        if nuevo_avanzar.x <= 0
            || nuevo_avanzar.y <= 0
            || nuevo_avanzar.x > self.cuadricula.0
            || nuevo_avanzar.y > self.cuadricula.1
            || self.serpiente.contains(&nuevo_avanzar)
        {
            self.fin_juego = true;
            return;
        }

        self.serpiente.push_front(nuevo_avanzar);

        if nuevo_avanzar == self.comida.posicion {
            self.puntaje += self.comida.valor;
            self.emitir_comida();
        } else {
            self.serpiente.pop_back();
        }
    }

    pub fn cambiar_direccion(&mut self, nueva_direccion: Direccion) {
        if !matches!(
            (nueva_direccion, self.direccion),
            (Direccion::Arriba, Direccion::Abajo)
                | (Direccion::Abajo, Direccion::Arriba)
                | (Direccion::Izquierda, Direccion::Derecha)
                | (Direccion::Derecha, Direccion::Izquierda)
        ) {
            self.direccion = nueva_direccion;
        }
    }

    pub fn es_fin_juego(&self) -> bool {
        self.fin_juego
    }

    pub fn puntaje(&self) -> i32 {
        self.puntaje
    }

    pub fn obtener_posicion_serpiente(&self) -> &LinkedList<Posicion> {
        &self.serpiente
    }

    pub fn obtener_posicion_comida(&self) -> Posicion {
        self.comida.posicion
    }

}
