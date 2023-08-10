pub mod todo_list;
use crate::todo_list::*;
use tezos_smart_rollup::{kernel_entry, prelude::Runtime};

pub fn entry<Host: Runtime>(host: &mut Host) {
    host.write_debug("Hello Kernel\n");
    execute(host);
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
                    execute(host, todo_list)
                }
                _ => execute(host, todo_list),
            }
        }
    }
}
#[cfg(test)]
mod tests {}
