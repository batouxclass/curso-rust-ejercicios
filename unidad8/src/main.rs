use crate::codigo_clase::mayor;
use crate::notificacion::notificacion;
use crate::resumen::Resumen;
use crate::sumar::add;
use crate::trait_creacion::{Archivo, Leer};

mod resumen;
mod notificacion;
mod sumar;
mod codigo_clase;
mod trait_creacion;

struct NuevoArticulo {
    titular: String,
    localizacion: String,
    autor: String,
    contenido: String,
}

struct Twitter {
    nombre_usuario: String,
    contenido: String,
}

impl Resumen for NuevoArticulo {
    fn resumir(&self) -> String {
        format!("{}, por: {} lugar: {}", self.titular, self.autor, self.localizacion)        
    }
}

impl Resumen for Twitter {
    fn resumir(&self) -> String {
        format!("{} : {}", self.nombre_usuario, self.contenido)
    }
}

fn main() {
    
    let resultado_mayor =  mayor(&[20.0,100.0, 1.0]);
    println!("Numero mayor {:?}", resultado_mayor);
    
    let archivo = Archivo;
    let result = archivo.leer().expect("No funciono");
    eprintln!("archivo uzie: {}", result);
    
    let articulo = NuevoArticulo {
        titular: String::from("Ballenas rascatadas en Japón"),
        contenido: String::from("Ballenas perdieron orientación se quedaron perdidas en la playa...."),
        autor: String::from("Clase Rust"),
        localizacion: String::from("Japón"),
    };
    
    println!("Noticia en desarrollo... {}", articulo.resumir());
    
    let tweet = Twitter {
        nombre_usuario: String::from("@Clase_Rust"),
        contenido: String::from("Ballenas perdieron orientación.... ver en link noticiero")
    };
    
    println!("Tweet resumir: {}", tweet.resumir());
    
    notificacion(&tweet);
    notificacion(&articulo);
    
    
    let suma = add(5.0, 1.0);
    println!("{}", suma);
    
}

