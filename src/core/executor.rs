use crate::core::constants::{ZOKRATES_BIN, ZOKRATES_PATH, ZPM_ZOKRATES_PATH};
use std::fmt;
use std::io::{stdin, Read, Write};
use std::path::PathBuf;
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
    pub fn to_vec(&self) -> Vec<&str> {
        match self.value {
            Some(value) => vec![self.key, value],
            None => vec![self.key],
        }
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
        write!(f, "{}", self.to_vec().join(" "))
    }
}

pub struct Executor {}

impl Executor {
    pub fn execute(cmd: Command, pipe_stdin: bool) -> Result<(), String> {
        let mut args = vec![cmd.name];
        args.append(
            cmd.args
                .iter()
                .map(|a| a.to_vec())
                .flatten()
                .collect::<Vec<&str>>()
                .as_mut(),
        );

        let zokrates_path = std::env::var(ZPM_ZOKRATES_PATH)
            .map(|p| PathBuf::from(p))
            .unwrap_or(dirs::home_dir().unwrap().join(ZOKRATES_PATH));

        debug!("{}={}", ZPM_ZOKRATES_PATH, zokrates_path.display());

        let zokrates_bin = std::fs::canonicalize(&zokrates_path)
            .map_err(|e| format!("{}: {}", zokrates_path.display(), e))?
            .join(ZOKRATES_BIN);

        info!("{} {}", ZOKRATES_BIN, cmd);

        let mut child = ProcessCommand::new(&zokrates_bin)
            .args(args)
            .stdin(Stdio::piped())
            .spawn()
            .map_err(|e| format!("{}: {}", zokrates_bin.display(), e))?;

        if pipe_stdin {
            debug!(
                "Executing child process with piped stdin (id: {})",
                child.id()
            );

            let mut stdin = stdin();
            let mut input = String::new();

            let child_stdin = child
                .stdin
                .as_mut()
                .ok_or(format!("Failed to open stdin"))?;

            match stdin.read_to_string(&mut input) {
                Ok(_) => {
                    debug!(
                        "Writing {} bytes to child stdin stream",
                        input.as_bytes().len()
                    );

                    child_stdin
                        .write_all(input.as_bytes())
                        .map_err(|e| format!("Failed to write to stdin: {}", e))?;

                    child_stdin
                        .flush()
                        .map_err(|e| format!("Could not flush stdin: {}", e))?;

                    Ok(())
                }
                Err(e) => Err(format!("Could not read stdin: {}", e)),
            }?;
        }

        let status = child
            .wait()
            .map_err(|_| "Could not get exit status from child process")?;

        if status.success() {
            debug!("Child process exited with code: {}", status.code().unwrap());
            Ok(())
        } else {
            match status.code() {
                Some(code) => Err(format!("Child process exited with code: {}", code)),
                None => Err("Child process terminated by signal".to_string()),
            }
        }
    }
}
