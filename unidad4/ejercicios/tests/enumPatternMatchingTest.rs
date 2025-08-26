#[cfg(test)]
mod project_management_tests {
    // Reutiliza tu 'enum' 'TaskStatus'.
    // 1. Define la `enum` 'TaskStatus' aquí.

    // 2. Define una función `get_status_message` que tome un `TaskStatus`
    //    y devuelva un `&str` (un slice de string).
    //    Usa la declaración `match` para manejar los diferentes estados:
    //    - 'Pending' -> "Esta tarea aún no ha sido iniciada."
    //    - 'InProgress' -> "La tarea está en curso. ¡Ánimo!"
    //    - 'Completed' -> "¡La tarea ha sido completada!"

    #[test]
    fn test_pattern_matching_with_enum() {
        // 3. Prueba la función con los 3 estados.
        let status_pending = TaskStatus::Pending;
        let message1 = get_status_message(status_pending);
        assert_eq!(message1, "Esta tarea aún no ha sido iniciada.");

        let status_in_progress = TaskStatus::InProgress;
        let message2 = get_status_message(status_in_progress);
        assert_eq!(message2, "La tarea está en curso. ¡Ánimo!");

        let status_completed = TaskStatus::Completed;
        let message3 = get_status_message(status_completed);
        assert_eq!(message3, "¡La tarea ha sido completada!");

        println!("Mensajes de estado correctos para todas las opciones.");
    }
}