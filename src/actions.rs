use std::str::from_utf8;

use crate::todo_list::TodoList;

pub enum Action {
    Add(String),
    SetDone(u32),
    SetUndone(u32),
}

impl TryFrom<Vec<u8>> for Action {
    type Error = String;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        match value.as_slice() {
            [0x00, remaining @ ..] => Ok(Action::Add(from_utf8(remaining).unwrap().to_string())),
            [0x01, b1, b2, b3, b4] => Ok(Action::SetDone(u32::from_be_bytes([*b1, *b2, *b3, *b4]))),
            [0x02, b1, b2, b3, b4] => {
                Ok(Action::SetUndone(u32::from_be_bytes([*b1, *b2, *b3, *b4])))
            }
            _ => Err("Invalid action".to_string()),
        }
    }
}

pub fn action_transition(todo_list: TodoList, action: Action) -> TodoList {
    match action {
        Action::Add(todo) => todo_list.add_todo(todo),
        Action::SetDone(id) => todo_list.set_done(id),
        Action::SetUndone(id) => todo_list.set_undone(id),
    }
}
