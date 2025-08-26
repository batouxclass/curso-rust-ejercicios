
pub mod notificador {

    pub trait Notificador {
        fn enviar(&self, mensaje: &str);
    }

    pub struct NotificadorBase;

    impl Notificador for NotificadorBase {
        fn enviar(&self, mensaje: &str) {
            println!("Notificando: {}", mensaje);
        }
    }

    pub struct DecoradorEmail<'a> {
        pub inner: &'a dyn Notificador,
    }

    impl<'a> Notificador for DecoradorEmail<'a> {
        fn enviar(&self, mensaje: &str) {
            self.inner.enviar(mensaje);
            println!("Enviando copia por Email: {}", mensaje);
        }
    }


}