use tezos_data_encoding::{enc::BinWriter, nom::NomReader};

use crate::todo_list::{Id, TodoList};

#[derive(Clone, PartialEq, Debug, BinWriter, NomReader)]
pub enum Action {
    Add(Title),
    SetDone(Id),
    SetUndone(Id),
}

#[derive(Clone, PartialEq, Debug, BinWriter, NomReader)]

pub struct Title {
    pub title: String,
}

impl TryFrom<Vec<u8>> for Action {
    type Error = String;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        match Action::nom_read(&value) {
            Ok((_, input)) => Ok(input),
            Err(_) => Err("Invalid action".to_string()),
        }
        // match value.as_slice() {
        //     [0x00, remaining @ ..] => Ok(Action::Add(from_utf8(remaining).unwrap().to_string())),
        //     [0x01, b1, b2, b3, b4] => Ok(Action::SetDone(u32::from_be_bytes([*b1, *b2, *b3, *b4]))),
        //     [0x02, b1, b2, b3, b4] => Ok(Action::SetUndone(SetUndone {
        //         field1: u32::from_be_bytes([*b1, *b2, *b3, *b4]),
        //     })),
        //     _ => Err("Invalid action".to_string()),
        // }
    }
}

pub fn action_transition(todo_list: TodoList, action: Action) -> TodoList {
    match action {
        Action::Add(todo) => todo_list.add_todo(todo.title),
        Action::SetDone(id) => todo_list.set_done(id.id),
        Action::SetUndone(id) => todo_list.set_undone(id.id),
    }
}
