pub mod actions;
pub mod todo_list;
use crate::actions::*;
use crate::todo_list::*;
use tezos_smart_rollup::storage::path::OwnedPath;
use tezos_smart_rollup::{kernel_entry, prelude::Runtime};

pub fn entry<Host: Runtime>(host: &mut Host) {
    let todo_list_next_id_path: OwnedPath =
        "/todo_list/next-id".as_bytes().to_vec().try_into().unwrap();

    let next_id = load_next_id(host, &todo_list_next_id_path);
    let todo_list = load_todo_list(host, &next_id);

    host.write_debug("Hello kernel\n");
    let new_todo_list = execute(host, todo_list.clone());

    save_todo_list(
        host,
        new_todo_list,
        todo_list,
        &todo_list_next_id_path,
        &next_id,
    );
}

fn load_next_id<Host: Runtime>(host: &mut Host, todo_list_next_id_path: &OwnedPath) -> Id {
    //Read next id to know if we will have to write new todo in storage

    host.store_read_all(todo_list_next_id_path)
        .map_err(|_| "Runtime error".to_string())
        .and_then(Id::try_from)
        .unwrap_or_default()
}

fn load_todo_list<Host: Runtime>(host: &mut Host, next_id: &Id) -> TodoList {
    let mut todo_list = TodoList::default();
    if next_id.id != 0 {
        for i in 0..next_id.id {
            //Read todos and insert in todo_list
            let todo_path: OwnedPath = format!("/todo_list/{i}")
                .as_bytes()
                .to_vec()
                .try_into()
                .unwrap();
            let todo = host
                .store_read_all(&todo_path)
                .map_err(|_| "Runtime error".to_string())
                .and_then(Todo::try_from)
                .unwrap();
            todo_list.todo_list.insert(i, todo);
        }
    }
    todo_list
}

fn save_todo_list<Host: Runtime>(
    host: &mut Host,
    new_todo_list: TodoList,
    old_todo_list: TodoList,
    todo_list_next_id_path: &OwnedPath,
    next_id: &Id,
) {
    //If there is new todo to the original list
    if new_todo_list.todo_list.len() > old_todo_list.todo_list.len() {
        //Write the next id in the storage to know which id to add next time
        let _ = host.store_write_all(todo_list_next_id_path, &((next_id.id + 1).to_be_bytes()));
        //Write the new todo in the storage
        let todo_path: OwnedPath = format!("/todo_list/{}", next_id.id)
            .as_bytes()
            .to_vec()
            .try_into()
            .unwrap();
        let todo: [u8; 8] = new_todo_list.todo_list[&next_id.id].clone().into();
        let _ = host.store_write_all(&todo_path, &todo);
    }

    if new_todo_list.todo_list != old_todo_list.todo_list {
        for (id, todo) in &old_todo_list.todo_list {
            if new_todo_list.todo_list[id] != *todo {
                //Write the new status of the todo in the storage
                let todo_path: OwnedPath = format!("/todo_list/{}", id)
                    .as_bytes()
                    .to_vec()
                    .try_into()
                    .unwrap();
                let todo: [u8; 8] = new_todo_list.todo_list[id].clone().into();
                let _ = host.store_write_all(&todo_path, &todo);
            }
        }
    }
}
kernel_entry!(entry);

fn execute<Host: Runtime>(host: &mut Host, todo_list: TodoList) -> TodoList {
    // Read the input
    let input = host.read_input();

    match input {
        // If it's an error or no messages then does nothing
        Err(_) | Ok(None) => todo_list,
        Ok(Some(message)) => {
            // If there is a message let's process it.
            let data = message.as_ref();
            match data {
                [0x00, ..] => {
                    host.write_debug("Message from the kernel.\n");
                    execute(host, todo_list)
                }
                [0x01, user_message @ ..] => {
                    host.write_debug("Message from the user.\n");
                    let action = Action::try_from(user_message.to_vec());
                    match action {
                        Ok(action) => {
                            let new_todo_list = action_transition(todo_list, action);
                            execute(host, new_todo_list)
                        }
                        Err(_) => execute(host, todo_list),
                    }
                }
                _ => execute(host, todo_list),
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        actions::{action_transition, Action, Title},
        todo_list::{Status, Todo, TodoList},
    };
    #[test]
    fn add_todo() {
        let user_message: [u8; 16] = [
            0, 0, 0, 0, 11, 87, 97, 115, 104, 32, 100, 105, 115, 104, 101, 115,
        ];
        let action = Action::try_from(user_message.to_vec());

        let expected_message = "Wash dishes".to_string();

        assert_eq!(
            action,
            Ok(Action::Add(Title {
                title: expected_message.clone()
            }))
        );

        let todo_list = TodoList::default();
        let new_todo_list = action_transition(todo_list, action.unwrap());

        let expected_todo_list: HashMap<u32, Todo> = HashMap::from([(
            0,
            Todo {
                title: expected_message,
                status: Status::Open,
            },
        )]);

        assert_eq!(
            new_todo_list,
            TodoList {
                todo_list: expected_todo_list
            }
        );
    }

    #[test]
    fn wrong_msg_test() {
        let user_message: [u8; 11] = [87, 97, 115, 104, 32, 100, 105, 115, 104, 101, 115];
        let action = Action::try_from(user_message.to_vec());

        assert_eq!(action, Err("Invalid action".to_string()));
    }
}
