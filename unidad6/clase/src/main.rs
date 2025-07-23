
fn main() {
    let mut nombres = vec![String::from("Ana"), String::from("Luis")];
    let otra_persona = &nombres[0];
    nombres.push(String::from("Juan"));
    
    // println!("{:?}", nombres);
    
    /// ejercicio
    
    let mut edades = vec![10, 20 , 30];
    edades.push(40);
    edades.pop();
    
    println!("{:?}", edades);
    
    //////////////
    
    let mut vacio: String = String::new();
    let datos: &str = "Contenido inicial";
    vacio = datos.to_string();
    println!("{:?}", vacio);
    
    /////////////////
    
    let mut saludo = String::from("Hola");
    saludo.push_str(", Ana");
    
    println!("{:?}", saludo);
    
    ///////////////////////

    let mut otro_saludo = String::from("Hello");
    let otro_dato = "Hola en español";
    otro_saludo = otro_dato.to_string();
    println!("{:?}", otro_saludo);
        
    ///////////////////////////////////
    
    let mut otro_vacio = String::new();
    otro_vacio.push_str("uniendome al string vacio");
    eprintln!("{}", otro_vacio);
}
