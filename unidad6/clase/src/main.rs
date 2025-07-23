use std::collections::HashMap;

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

    ///////////////////////////////////

    let mut nombres = vec![String::from("Luis"), String::from("Ana")];
    let mut cont = 0;
    for nombre in &nombres {
        cont = cont+1;
        if nombre == &String::from("Ana") {
            println!("hola {} en cont={}", nombre, cont );
            break;
        }
    }

    /////////////////////////////////

    let mut edades = HashMap::new();
    edades.insert("Ana", 30);
    edades.insert("Luis", 25);

    for (key, value) in &edades {
        eprintln!("key: {} valor:  {}", key, value);
    }

    ////////////////////////////////

    let edad_ana = edades.get("Ana").unwrap();
    println!("Edad Ana: {}", edad_ana);

    ////////////////////////////////

    match edades.get("Pedro") {
        Some(edad) =>  println!("Edad Pedro en match: {}", edad),
        None => println!("Pedro no esta en el hashmap"),
    }

    /////////////////////////////////////

    let mut edades = HashMap::new();
    edades.insert("Ana", 30);
    edades.insert("Luis", 25);
    edades.entry("Juan").or_insert(27);

    println!("{:?}", edades);


    /////////////////////////////////////

    let nombre = String::from("Ana");
    let edad = 30;
    
    let mut personas = HashMap::new();
    personas.insert(nombre, edad);
    
    println!("{:?}", personas);
    
    /////////////////////////////////////
    
    let mut puntaje = HashMap::new();
    puntaje.insert("Juan", 26);
    puntaje.insert("Ana", 25);
    puntaje.entry("Juan").or_insert(35);
    println!("{:?}", puntaje);
    
}
