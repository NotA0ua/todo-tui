pub struct Todo {
    pub text: String,
    pub completed: bool,
}

impl Default for Todo {
    fn default() -> Self {
        Todo {
            text: String::new(),
            completed: false,
        }
    }
}
