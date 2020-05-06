use crate::core::constants::ZOKRATES_BIN;
use std::fmt;
use std::io::{stdin, Read, Write};
use std::process::{Command as ProcessCommand, Stdio};

pub struct Command<'a> {
    name: &'a str,
    args: Vec<Argument<'a>>,
}

pub struct Argument<'a> {
    key: &'a str,
    value: Option<&'a str>,
}

impl<'a> Command<'a> {
    pub fn new(name: &'a str, args: Vec<Argument<'a>>) -> Self {
        Command { name, args }
    }
}

impl<'a> Argument<'a> {
    pub fn new(key: &'a str, value: Option<&'a str>) -> Self {
        Argument { key, value }
    }
}

impl fmt::Display for Command<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}",
            self.name,
            self.args
                .iter()
                .map(|a| format!("{}", a))
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

impl fmt::Display for Argument<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.value {
            Some(_) => write!(f, "{} {}", self.key, self.value.unwrap()),
            None => write!(f, "{}", self.key),
        }
    }
}

pub struct Executor {}

impl Executor {
    pub fn execute(cmd: Command, pipe_stdin: bool) -> Result<(), String> {
        let mut args = vec![cmd.name];
        args.append(
            cmd.args
                .iter()
                .map(|a| match a.value {
                    Some(_) => vec![a.key, a.value.unwrap()],
                    None => vec![a.key],
                })
                .flatten()
                .collect::<Vec<&str>>()
                .as_mut(),
        );

        info!("{} {}", ZOKRATES_BIN, cmd);

        let mut child = ProcessCommand::new(ZOKRATES_BIN)
            .args(args)
            .stdin(Stdio::piped())
            .spawn()
            .expect("Could not spawn child process");

        if pipe_stdin {
            let mut stdin = stdin();
            let mut input = String::new();
            let child_stdin = child.stdin.as_mut().expect("Failed to open stdin");

            match stdin.read_to_string(&mut input) {
                Ok(_) => {
                    child_stdin
                        .write_all(input.as_bytes())
                        .expect("Failed to write to stdin");
                    child_stdin.flush().expect("Could not flush stdin");
                    Ok(())
                }
                Err(e) => Err(format!("Could not read stdin: {}", e)),
            }?;
        }

        let status = child
            .wait()
            .expect("Could not get exit status from child process");

        if status.success() {
            Ok(())
        } else {
            match status.code() {
                Some(code) => Err(format!("Exited with status code: {}", code)),
                None => Err("Process terminated by signal".to_string()),
            }
        }
    }
}
