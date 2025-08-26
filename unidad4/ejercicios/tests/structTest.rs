#[cfg(test)]
mod project_management_tests {
    // 1. Define aquí la 'struct' 'Task'.
    // Debe tener los siguientes campos:
    // - 'title': String
    // - 'description': String
    // - 'status': String 

    #[test]
    fn test_create_task() {
        // 2. Crea una nueva instancia de 'Task' con datos de ejemplo.
        // Asigna el 'title', 'description' y 'status' de la tarea.
        // Por ejemplo: "Planificar", "Dividir el proyecto en fases", "Pendiente"
        let my_task = /* AQUI FALTA TU CODIGO */;

        // 3. Comprueba que los datos de la instancia son correctos.
        assert_eq!(my_task.title, "Planificar");
        assert_eq!(my_task.description, "Dividir el proyecto en fases");
        assert_eq!(my_task.status, "Pendiente");

        println!("Tarea creada con éxito: '{}'", my_task.title);
    }
}