use std::{io, thread::sleep, time::Duration};

use crossterm::{
    cursor::MoveTo,
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use rpassword::read_password;

use crate::Result;

pub fn read_line(stdout: &mut io::Stdout, stdin: &io::Stdin, prefix: &str) -> Result<String> {
    let mut input = String::new();

    let mut is_not_empty = false;

    while !is_not_empty {
        execute!(
            stdout,
            SetForegroundColor(Color::Yellow),
            Print(format!("{}: ", prefix))
        )?;
        stdin.read_line(&mut input)?;
        input = input.trim().to_string();

        if !input.is_empty() {
            is_not_empty = true;
        } else {
            execute!(
                stdout,
                SetForegroundColor(Color::Yellow),
                Print(format!("{} cannot be empty!\n", prefix))
            )?;
        }
    }

    Ok(input)
}

pub fn read_line_hidden(stdout: &mut io::Stdout, prefix: &str) -> Result<String> {
    let mut input = String::new();

    let mut is_not_empty = false;

    while !is_not_empty {
        execute!(
            stdout,
            SetForegroundColor(Color::Yellow),
            Print(format!("{}: ", prefix))
        )?;
        input = read_password()?;
        println!("{}", input);
        input = input.trim().to_string();

        if !input.is_empty() {
            is_not_empty = true;
        } else {
            execute!(
                stdout,
                SetForegroundColor(Color::Yellow),
                Print(format!("{} cannot be empty!\n", prefix))
            )?;
        }
    }

    Ok(input)
}

pub fn read_line_optional(
    stdout: &mut io::Stdout,
    stdin: &io::Stdin,
    prefix: &str,
) -> Result<Option<String>> {
    let mut input = String::new();

    execute!(
        stdout,
        SetForegroundColor(Color::Yellow),
        Print(format!("{}: ", prefix))
    )?;
    stdin.read_line(&mut input)?;
    input = input.trim().to_string();

    if input.is_empty() {
        return Ok(None);
    }

    Ok(Some(input))
}

pub fn print_system_blue(stdout: &mut io::Stdout, msg: &str) -> Result<()> {
    execute!(
        stdout,
        SetForegroundColor(Color::Blue),
        Print(format!("{}\n", msg))
    )?;

    Ok(())
}

pub fn print_system_green(stdout: &mut io::Stdout, msg: &str) -> Result<()> {
    execute!(
        stdout,
        SetForegroundColor(Color::Green),
        Print(format!("{}\n", msg))
    )?;

    Ok(())
}

pub fn print_error(stdout: &mut io::Stdout, msg: &str) -> Result<()> {
    execute!(
        stdout,
        SetForegroundColor(Color::Red),
        Print(format!("{}\n", msg))
    )?;

    Ok(())
}

pub fn system_wait(seconds: u64) {
    sleep(Duration::from_secs(seconds));
}

pub fn system_clear_terminal(stdout: &mut io::Stdout) -> Result<()> {
    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

    Ok(())
}
