pub mod db;
pub mod error;
pub mod handlers;
pub mod schema;
pub mod setup;
pub mod state;
pub mod util;

pub use db::*;
pub use error::*;
pub mod model;
pub use handlers::*;
use handlers::{
    add::add_contact, edit::edit_contact, list::list_contacts, remove::remove_contact,
    search::search_contacts,
};
pub use state::*;
pub use util::*;

use std::io::{self, Write};

pub fn start(stdout: &mut io::Stdout, stdin: &io::Stdin) -> Result<()> {
    let state = match setup::setup(stdout, stdin) {
        Ok(state) => state,
        Err(_) => panic!("Console error!!"),
    };

    // -- start main loop
    if main_loop(state, stdin, stdout).is_err() {
        panic!("Console error!!");
    }

    Ok(())
}

fn main_loop(state: AppState, stdin: &io::Stdin, stdout: &mut io::Stdout) -> Result<()> {
    loop {
        // Print the prompt
        print_system_blue(
            stdout,
            "Commands available: list(l), add (a), edit (e), remove (r), search (s), quit (q).\n",
        )?;
        print_system_blue(stdout, "Command > ")?;
        stdout.flush()?;

        let mut input = String::new();
        // Read a line from the user
        stdin.read_line(&mut input)?;
        let input = input.trim();

        // If the user types "quit", break out of the loop.
        if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("q") {
            print_system_green(stdout, "Exiting...\n")?;
            break;
        }

        // Process the command.
        // You can add more commands and logic here that uses state.pool, for instance.
        match input {
            "list" | "l" => {
                if list_contacts(state.clone(), stdout, stdin).is_err() {
                    print_error(stdout, "Could not list contacts!")?;
                    system_wait(1);
                    system_clear_terminal(stdout)?;
                }
            }
            "add" | "a" => {
                if add_contact(state.clone(), stdout, stdin).is_err() {
                    print_error(stdout, "Could not add new contact!")?;
                    system_wait(1);
                    system_clear_terminal(stdout)?;
                }
            }
            "edit" | "e" => {
                if edit_contact(state.clone(), stdout, stdin).is_err() {
                    print_error(stdout, "Could not edit contact!")?;
                    system_wait(1);
                    system_clear_terminal(stdout)?;
                }
            }
            "remove" | "r" => {
                if remove_contact(state.clone(), stdout, stdin).is_err() {
                    print_error(stdout, "Could not remove contact!")?;
                    system_wait(1);
                    system_clear_terminal(stdout)?;
                }
            }
            "search" | "s" => {
                if search_contacts(state.clone(), stdout, stdin).is_err() {
                    print_error(stdout, "Could not search for contacts!")?;
                    system_wait(1);
                    system_clear_terminal(stdout)?;
                }
            }
            _ => {
                print_error(stdout, "Unknown command.\n")?;
            }
        }
    }

    Ok(())
}
