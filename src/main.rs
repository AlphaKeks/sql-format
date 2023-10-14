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

	Ok(())
}

/// This program formats SQL code.
#[derive(Parser)]
struct Args {
	/// Read the input from a file instead of stdin.
	#[arg(short, long, value_name = "PATH")]
	input_file: Option<PathBuf>,

	/// Write the output to a file instead of stdout.
	#[arg(short, long, value_name = "PATH")]
	output_file: Option<PathBuf>,

	/// Use spaces for indentation.
	#[arg(long, value_name = "AMOUNT")]
	spaces: Option<u8>,

	/// Use lowercase for SQL keywords.
	#[arg(long)]
	lowercase: bool,
}
