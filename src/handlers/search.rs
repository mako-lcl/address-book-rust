use crate::{
    AppState, Result, contacts::search_contacts_by_term, model::FormatTable, print_system_green,
    read_line, system_clear_terminal,
};
use std::io;

pub fn search_contacts(state: AppState, stdout: &mut io::Stdout, stdin: &io::Stdin) -> Result<()> {
    system_clear_terminal(stdout)?;

    print_system_green(stdout, "--- Searching contacts ---\n")?;

    // -- Get search term
    let search_term = read_line(stdout, stdin, "Search Term")?;

    // -- search contacts
    let contacts = state.with_conn(|conn| search_contacts_by_term(conn, search_term))?;

    // -- print contacts
    print_system_green(stdout, &format!("\n{}\n", contacts.format_to_table()))?;
    print_system_green(stdout, "Print enter to return...")?;
    stdin.read_line(&mut String::new())?;
    system_clear_terminal(stdout)?;

    Ok(())
}
