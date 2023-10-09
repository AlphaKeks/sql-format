// Copyright (C) AlphaKeks <alphakeks@dawn.sh>
//
// This is free software. You can redistribute it and / or modify it under the terms of the
// GNU General Public License as published by the Free Software Foundation, either version 3
// of the License, or (at your option) any later version.
//
// You should have received a copy of the GNU General Public License along with this repository.
// If not, see <https://www.gnu.org/licenses/>.

use {
	clap::Parser,
	color_eyre::{eyre::Context, Result},
	sqlformat::{FormatOptions, Indent, QueryParams},
	std::{
		io::{IsTerminal, Read, Write},
		path::PathBuf,
	},
};

fn main() -> Result<()> {
	color_eyre::install()?;

	let args = Args::parse();
	let mut input = String::new();

	if let Some(file) = args.input_file {
		input = std::fs::read_to_string(file).context("Failed to read file.")?;
	} else if std::io::stdin().is_terminal() {
		std::io::stdin()
			.read_line(&mut input)
			.context("Failed to read STDIN.")?;
	} else {
		std::io::stdin()
			.read_to_string(&mut input)
			.context("Failed to read STDIN.")?;
	}

	eprintln!("Read input.");

	let mut output = sqlformat::format(&input, &QueryParams::None, FormatOptions {
		indent: match args.spaces {
			None => Indent::Tabs,
			Some(count) => Indent::Spaces(count),
		},
		uppercase: !args.lowercase,
		lines_between_queries: 2,
	});

	if std::io::stdout().is_terminal() {
		output.push('\n');
	}

	if let Some(file) = args.output_file {
		std::fs::write(file, output).context("Failed to write to file.")?;
	} else {
		std::io::stdout()
			.write_all(output.as_bytes())
			.context("Failed to write to STDOUT.")?;
	}

	eprintln!("Wrote output.");

	Ok(())
}

#[derive(Parser)]
struct Args {
	/// File to read input from.
	///
	/// If this is not specified, STDIN will be used instead.
	#[arg(short, long, value_name = "PATH")]
	input_file: Option<PathBuf>,

	/// File to write output to.
	///
	/// If this is not specified, STDOUT will be used instead.
	#[arg(short, long, value_name = "PATH")]
	output_file: Option<PathBuf>,

	/// Whether to use spaces for indentation.
	#[arg(long)]
	spaces: Option<u8>,

	/// Whether to use lowercase for SQL keywords.
	#[arg(long)]
	lowercase: bool,
}
