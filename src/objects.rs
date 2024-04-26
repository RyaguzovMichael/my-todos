use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Task{
    title: String,
    description: Option<String>,
    parent: Option<Weak<TodoList>>,
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title && self.description == other.description
    }
}

impl Task {
    fn new(title: &str) -> Self{
        Task {
            title: title.to_string(),
            description: None,
            parent: None,
        }
    }
}

struct TodoList{
    title: String,
    tasks: Vec<Rc<Task>>,
}

impl TodoList {
    fn new(title: &str) -> Self{
        TodoList{
            title: title.to_string(),
            tasks: Vec::new(),
        }
    }

    fn add(&mut self, task: Rc<Task>){
        self.tasks.push(task)
    }

    fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_add_task_to_task_list(){
        let task = Rc::new(Task::new("Test task"));
        {
            let mut todo_list = TodoList::new("Test todo list");
            todo_list.add(Rc::clone(&task));
            assert_eq!(todo_list.tasks.first(), Some(&task.clone()));
            assert_eq!(Rc::strong_count(&task), 2);
        }

        assert_eq!(Rc::strong_count(&task), 1);
    }

    #[test]
    fn todo_list_is_empty_initially() {
        let todo_list = TodoList::new("Test todo list");
        assert!(todo_list.is_empty());
    }

    #[test]
    fn todo_list_not_empty_after_adding_task() {
        let task = Rc::new(Task::new("Test task"));
        let mut todo_list = TodoList::new("Test todo list");
        todo_list.add(Rc::clone(&task));
        assert!(!todo_list.is_empty());
    }
}