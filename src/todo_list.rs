use std::collections::HashMap;

pub struct TodoList {
    pub todo_list: HashMap<u32, Todo>,
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
            todo_list: HashMap::new(),
        }
    }
}

impl TodoList {
    fn add_todo(self, title: String) -> TodoList {
        let mut id = 0;
        if !(self.todo_list.is_empty()) {
            id = self.todo_list.into_keys().max().unwrap() + 1;
        }
        let mut new_todo_list = self.todo_list;
        new_todo_list.insert(
            id,
            Todo {
                title,
                status: Status::Open,
            },
        );
        TodoList {
            todo_list: new_todo_list,
        }
    }

    fn set_done(self, pos: u32) -> TodoList {
        let mut new_todo_list = self.todo_list;
        new_todo_list.insert(pos, new_todo_list[&pos].set_done());
        TodoList {
            todo_list: new_todo_list,
        }
    }

    fn set_undone(self, pos: u32) -> TodoList {
        let mut new_todo_list = self.todo_list;
        new_todo_list.insert(pos, new_todo_list[&pos].set_undone());
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

pub enum Action {
    Add(String),
    SetDone(u32),
    SetUndone(u32),
}
