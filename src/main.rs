fn main() -> Result<(),std::io::Error> {
	if let Some(crate::Diagnostic{message, spans, rendered: Some(rendered), ..}, ..) = build(std::env::args().skip(1))? {
		eprint!("{}", rendered);
		for span in spans {
			if std::path::Path::new(&span.file_name).exists() { println!("{}:{}:{}", span.file_name, span.line_start, span.column_start); }
		}
		std::process::exit(-1);
	}
	std::process::exit(child.wait()?.code().unwrap_or(-1));
}
