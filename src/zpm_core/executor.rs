use std::fmt;
use std::process::Command as ProcessCommand;

pub struct Command<'a> {
    name: &'a str,
    args: Vec<Argument<'a>>,
}

pub struct Argument<'a> {
    key: &'a str,
    value: &'a str,
}

impl<'a> Command<'a> {
    pub fn new(name: &'a str, args: Vec<Argument<'a>>) -> Self {
        Command { name, args }
    }
}

impl<'a> Argument<'a> {
    pub fn new(key: &'a str, value: &'a str) -> Self {
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
        write!(f, "{} {}", self.key, self.value)
    }
}

pub struct Executor {}

impl Executor {
    pub fn execute(cmd: Command) -> Result<(), String> {
        let mut args = vec![cmd.name];
        args.append(
            cmd.args
                .iter()
                .map(|a| vec![a.key, a.value])
                .flatten()
                .collect::<Vec<&str>>()
                .as_mut(),
        );

        info!("zokrates {}", cmd);

        let mut child = ProcessCommand::new("zokrates")
            .args(args)
            .spawn()
            .expect("Could not spawn child process");

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
