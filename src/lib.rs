#![feature(associated_type_bounds)]
mod diagnostic;
pub use diagnostic::{Diagnostic, Span};

use std::process::{Command, Stdio};
#[fehler::throws(std::io::Error)] pub fn build(args: impl IntoIterator<Item:AsRef<std::ffi::OsStr>>) -> Option<Diagnostic> {
	let mut child = Command::new("cargo").arg("build").args(args).arg("--message-format=json").stdout(Stdio::piped()).spawn()?;
	use diagnostic::{parse, Message, CompilerMessage};
	for msg in parse(std::io::BufReader::new(child.stdout.take().unwrap())) { match msg? {
		Message::CompilerMessage(CompilerMessage{message, ..}) => {
			let _ = child.kill(); // Kill on first warning/error to save energy/heat
			if message.message == "aborting due to previous error" { continue; }
			return Some(message);
		},
		_=>{},
	}}
	None
}
