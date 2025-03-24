use crate::{
    AppState, Result,
    contacts::{delete_contact, find_contact_by_name},
    print_error, print_system_green, read_line, system_clear_terminal, system_wait,
};
use std::io;

pub fn remove_contact(state: AppState, stdout: &mut io::Stdout, stdin: &io::Stdin) -> Result<()> {
    system_clear_terminal(stdout)?;
    print_system_green(stdout, "--- Removing contact ---")?;

    // -- find contact
    let first_name = read_line(stdout, stdin, "First Name")?;
    let last_name = read_line(stdout, stdin, "Last Name")?;

    let contact = match state.with_conn(|conn| find_contact_by_name(conn, first_name, last_name)) {
        Ok(contact) => contact,
        Err(_) => {
            print_error(stdout, "No contact with this name!")?;
            system_wait(1);
            system_clear_terminal(stdout)?;
            return Ok(());
        }
    };

    state.with_conn(|conn| delete_contact(conn, contact))?;

    print_system_green(stdout, "Succesfully deleted contact!")?;
    system_wait(1);
    system_clear_terminal(stdout)?;

    Ok(())
}
