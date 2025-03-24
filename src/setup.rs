use crate::{
    AppState, Result, connect, establish_connection, print_system_blue, system_clear_terminal,
};
use std::io;

pub fn setup(stdout: &mut io::Stdout, stdin: &io::Stdin) -> Result<AppState> {
    greeting(stdout)?;
    let config = connect(stdout, stdin)?;
    let pool = establish_connection(stdout, config.database_url())?;

    // -- clear terminal
    system_clear_terminal(stdout)?;

    Ok(AppState::new(pool))
}

fn greeting(stdout: &mut io::Stdout) -> Result<()> {
    system_clear_terminal(stdout)?;

    print_system_blue(
        stdout,
        "Welcome to Hermes Address Book!\nYou can add, edit, search, delete and show contacts.\nBut first you will need to connect to a postgres database.\n",
    )?;

    Ok(())
}
