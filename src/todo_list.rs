use std::{collections::HashMap, str::from_utf8};

#[derive(Clone, Default)]

pub struct TodoList {
    pub todo_list: HashMap<u32, Todo>,
}

#[derive(PartialEq, Clone)]
pub struct Todo {
    pub title: String,
    pub status: Status,
}

#[derive(PartialEq, Clone)]

pub enum Status {
    Open,
    Achieved,
}

impl TodoList {
    pub fn add_todo(self, title: String) -> TodoList {
        let mut id = 0;
        if !(self.todo_list.is_empty()) {
            id = self.todo_list.clone().into_keys().max().unwrap() + 1;
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

    pub fn set_done(self, id: u32) -> TodoList {
        let mut new_todo_list = self.todo_list;
        new_todo_list.insert(id, new_todo_list[&id].clone().set_done());
        TodoList {
            todo_list: new_todo_list,
        }
    }

    pub fn set_undone(self, id: u32) -> TodoList {
        let mut new_todo_list = self.todo_list;
        new_todo_list.insert(id, new_todo_list[&id].clone().set_undone());
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

impl TryFrom<Vec<u8>> for Todo {
    type Error = String;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        match value.as_slice() {
            [0x01, remaining @ ..] => Ok(Todo {
                title: from_utf8(remaining).unwrap().to_string(),
                status: Status::Achieved,
            }),
            [0x02, remaining @ ..] => Ok(Todo {
                title: from_utf8(remaining).unwrap().to_string(),
                status: Status::Open,
            }),
            _ => Err("Invalid deserialization".to_string()),
        }
    }
}

impl From<Todo> for [u8; 8] {
    fn from(val: Todo) -> Self {
        let title_as_bytes = val.title.as_bytes().to_vec();
        let mut vec_of_byte = match val.status {
            Status::Open => vec![0x02],
            Status::Achieved => vec![0x01],
        };
        vec_of_byte.extend(title_as_bytes);
        vec_of_byte.as_slice().try_into().unwrap()
    }
}

#[derive(Default)]
pub struct Id {
    pub id: u32,
}

impl TryFrom<Vec<u8>> for Id {
    type Error = String;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        value
            .try_into()
            .map_err(|_| "Error".to_string())
            .map(u32::from_be_bytes)
            .map(|id| Id { id })
    }
}
