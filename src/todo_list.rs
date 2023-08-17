use std::collections::HashMap;

use tezos_data_encoding::{enc::BinWriter, nom::NomReader};

#[derive(Clone, PartialEq, Debug, Default)]

pub struct TodoList {
    pub todo_list: HashMap<u32, Todo>,
}

#[derive(PartialEq, Clone, Debug, NomReader, BinWriter)]
pub struct Todo {
    pub title: String,
    pub status: Status,
}

#[derive(PartialEq, Debug, Clone, NomReader, BinWriter)]

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
        match Todo::nom_read(&value) {
            Ok((_, input)) => Ok(input),
            Err(_) => Err("Invalid deserialization".to_string()),
        }
    }
}

impl From<Todo> for [u8; 8] {
    fn from(val: Todo) -> Self {
        let mut bytes = Vec::new();
        val.bin_write(&mut bytes).ok();
        bytes.try_into().unwrap()
    }
}

#[derive(Default, Clone, PartialEq, Debug, BinWriter, NomReader)]
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
