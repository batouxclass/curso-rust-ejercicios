pub mod domain {
    #[derive(Debug)]
    pub struct UserId(String);
    
    impl UserId {
        pub fn new(id: &str) -> Option<Self> {
            if (!id.is_empty()) {
                Some(UserId(id.to_string()))
            } else {
                None
            }
        }
        
        pub fn get_id(&self) -> &str {
            &self.0
        }
    }
}