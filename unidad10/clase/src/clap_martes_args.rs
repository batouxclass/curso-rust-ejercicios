use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct MartesArgs {
    pub cita: String,
    pub ubicacion: String,
}
/*
fn main() {
    let calendario = MartesArgs::parse();
    println!("Cita: {:?}", calendario.cita);
    println!("Ubicacion: {:?}", calendario.ubicacion);
}
*/