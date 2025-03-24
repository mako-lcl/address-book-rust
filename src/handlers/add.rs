use crate::{
    AppState, Result,
    contacts::{find_contact_by_name, insert_contact},
    model::NewContact,
    print_error, print_system_green, read_line, read_line_optional, system_clear_terminal,
    system_wait,
};
use std::io;

pub fn add_contact(state: AppState, stdout: &mut io::Stdout, stdin: &io::Stdin) -> Result<()> {
    system_clear_terminal(stdout)?;

    print_system_green(stdout, "--- Adding a new contact ---\n")?;

    // -- Get contact info
    let first_name = read_line(stdout, stdin, "First Name")?;
    let last_name = read_line(stdout, stdin, "Last Name")?;

    if state
        .with_conn(|conn| find_contact_by_name(conn, first_name.clone(), last_name.clone()))
        .is_ok()
    {
        print_error(stdout, "Cannot add a duplicate contact!")?;
        system_wait(1);
        system_clear_terminal(stdout)?;
        return Ok(());
    }

    let phone_number = read_line_optional(stdout, stdin, "Phone Number")?;
    let email = read_line_optional(stdout, stdin, "Email")?;

    // -- create new contact
    let new_contact = NewContact {
        first_name,
        last_name,
        phone_number,
        email,
    };
    state.with_conn(|conn| insert_contact(conn, new_contact))?;

    print_system_green(stdout, "Succesfully added contact!")?;
    system_wait(1);
    system_clear_terminal(stdout)?;

    Ok(())
}
