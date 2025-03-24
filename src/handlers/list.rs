use crate::{
    AppState, contacts::select_contacts, error::Result, model::FormatTable, print_system_green,
    system_clear_terminal,
};
use std::io;

pub fn list_contacts(state: AppState, stdout: &mut io::Stdout, stdin: &io::Stdin) -> Result<()> {
    system_clear_terminal(stdout)?;
    print_system_green(stdout, "--- Listing contacts ---\n")?;

    // -- find contacts
    let contacts = state.with_conn(|conn| {
        let res = select_contacts(conn)?;

        Ok(res)
    })?;

    // -- print contacts
    print_system_green(stdout, &format!("\n{}\n", contacts.format_to_table()))?;
    print_system_green(stdout, "Print enter to return...")?;
    stdin.read_line(&mut String::new())?;
    system_clear_terminal(stdout)?;

    Ok(())
}
