//! Small helpers around spawning `git`, shared by the steps that stage and commit.

use log::info;
use nobubbles::inline::Report;
use std::io::{BufRead, BufReader, Read};
use std::process::{Command, ExitStatus, Stdio};
use std::sync::mpsc::{channel, Sender};
use std::thread;

/// Runs `command` (program, then its args) to the end, handing every line it prints to
/// `report` as it happens.
///
/// Both pipes are drained on their own threads. Reading one only after the other has
/// finished is how a pipe fills up and both sides stop for good, and git talks on both.
pub fn run(command: &[String], report: &Report) -> std::io::Result<ExitStatus> {
    info!(target: "tui::git", "running git command: {:?}", command);
    let (program, args) = command.split_first().expect("command can't be empty");

    let mut child = Command::new(program)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let (tx, rx) = channel();
    pipe(child.stdout.take(), &tx);
    pipe(child.stderr.take(), &tx);
    drop(tx); // the readers hold the last senders, so `rx` ends when both pipes close

    for line in rx {
        report.say(line);
    }

    child.wait()
}

/// The same, with no UI around it, for quick reads like `git status`.
pub fn quietly(args: &[&str]) -> Vec<String> {
    let Ok(output) = Command::new("git").args(args).output() else {
        return Vec::new();
    };

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::to_owned)
        .collect()
}

fn pipe(stream: Option<impl Read + Send + 'static>, tx: &Sender<String>) {
    let Some(stream) = stream else { return };
    let tx = tx.clone();

    thread::spawn(move || {
        for line in BufReader::new(stream).lines().map_while(Result::ok) {
            let _ = tx.send(line);
        }
    });
}
