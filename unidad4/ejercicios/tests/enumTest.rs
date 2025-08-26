#[cfg(test)]
mod project_management_tests {
    // 1. Define aquí la 'enum' 'TaskStatus'.
    // Los estados posibles son:
    // - 'Pending'
    // - 'InProgress'
    // - 'Completed'

    #[test]
    fn test_enum_task_status() {
        // 2. Crea una variable con el estado 'InProgress' usando tu `enum`.
        let current_status = /* AQUI FALTA TU CODIGO */;

        // 3. Asegúrate de que el estado es el correcto.
        // Si sabes cómo, puedes usar 'if let' o un 'match' para la comprobación.
        // De lo contrario, puedes simplemente dejar el código como está, ya que la asignación es una buena prueba.

        // Aquí se comprueba que la variable 'current_status' sea de tipo 'TaskStatus::InProgress'
        if let TaskStatus::InProgress = current_status {
            println!("El estado de la tarea es 'En Progreso'. ¡Correcto!");
            assert!(true);
        } else {
            assert!(false, "El estado no es el esperado.");
        }
    }
}