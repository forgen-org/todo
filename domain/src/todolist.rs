#[derive(Debug, Default)]
pub struct TodoList {
    pub tasks: Vec<Task>,
}

#[derive(Debug)]
pub struct Task {
    pub name: String,
    pub completed: Completed,
}

#[derive(Debug, PartialEq)]
pub enum Completed {
    Yes,
    No,
}
