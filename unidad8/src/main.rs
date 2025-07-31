trait Resumen {
    fn resumir(&self) -> String;
}

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
}

