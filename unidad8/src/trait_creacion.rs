use crate::Twitter;

#[derive(Debug)]
pub struct  Archivo;

pub trait Leer {
    fn leer(self: &Self) -> Result<usize, String> {
        Ok(100)
    }
}

impl Leer for Archivo {
    fn leer(self: &Self) -> Result<usize, String> {
        Ok(2500)
    }
}