use crate::{
    AppState, Result,
    contacts::{find_contact_by_name, update_contact},
    print_error, print_system_green, read_line, read_line_optional, system_clear_terminal,
    system_wait,
};
use std::io;

pub fn edit_contact(state: AppState, stdout: &mut io::Stdout, stdin: &io::Stdin) -> Result<()> {
    system_clear_terminal(stdout)?;

    print_system_green(stdout, "--- Updating a new contact ---\n")?;

    // -- find contact
    let first_name = read_line(stdout, stdin, "First Name")?;
    let last_name = read_line(stdout, stdin, "Last Name")?;

    let mut contact =
        match state.with_conn(|conn| find_contact_by_name(conn, first_name, last_name)) {
            Ok(contact) => contact,
            Err(_) => {
                print_error(stdout, "No contact with this name!")?;
                system_wait(1);
                system_clear_terminal(stdout)?;
                return Ok(());
            }
        };

    // -- update contact
    let new_first_name = read_line(stdout, stdin, "New First Name")?;
    let new_last_name = read_line(stdout, stdin, "New Last Name")?;
    let phone_number = read_line_optional(stdout, stdin, "Phone Number")?;
    let email = read_line_optional(stdout, stdin, "Email")?;
    contact.first_name = new_first_name;
    contact.last_name = new_last_name;
    contact.phone_number = phone_number;
    contact.email = email;

    state.with_conn(|conn| update_contact(conn, contact))?;

    print_system_green(stdout, "Succesfully edited contact!")?;
    system_wait(1);
    system_clear_terminal(stdout)?;

    Ok(())
}
