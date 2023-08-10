pub struct TodoList {
    pub todo_list: Vec<Todo>,
}

pub struct Todo {
    pub title: String,
    pub status: Status,
}

enum Status {
    Open,
    Achieved,
}

impl Default for TodoList {
    fn default() -> TodoList {
        TodoList {
            todo_list: Vec::new(),
        }
    }
}

impl TodoList {
    fn add_todo(self, title: String) -> TodoList {
        let mut new_todo_list = self.todo_list;
        new_todo_list.push(Todo {
            title,
            status: Status::Open,
        });
        TodoList {
            todo_list: new_todo_list,
        }
    }
}

impl Todo {
    fn set_done(self) -> Todo {
        Todo {
            title: self.title,
            status: Status::Achieved,
        }
    }
    fn set_undone(self) -> Todo {
        Todo {
            title: self.title,
            status: Status::Open,
        }
    }
}
