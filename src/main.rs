use std::collections::HashMap;

use clap::Parser;
use colored::ColoredString;
use colored::Colorize;
use isahc::ReadResponseExt;
use linku_sona::{UsageCategory, Word};

mod error;

#[derive(serde::Deserialize)]
#[serde(untagged)]
enum ApiResult {
	Word(Box<Word>),
	Error { message: String },
}

#[derive(Parser)]
struct Cli {
	/// Show the RAW JSON response from the API
	#[clap(short = 'j', long)]
	json: bool,
	/// The language used to get the word definitions.
	#[clap(short = 't', long, default_value = "en")]
	toki: String,
	/// The word to get the definition of
	word: String,
}

fn main() -> Result<(), error::Error> {
	let cli = Cli::parse();

	let url = format!(
		"https://api.linku.la/v1/words/{}?lang={}",
		cli.word, cli.toki
	);

	let response_string = isahc::get(url)?.text()?;

	if cli.json {
		println!("{}", response_string);
		return Ok(());
	}

	let response: ApiResult = serde_json::from_str(&response_string)?;
	match response {
		ApiResult::Word(word) => show(word, cli.toki),
		ApiResult::Error { message } => eprintln!("Error: {}", message),
	}

	Ok(())
}

fn show(word: Box<Word>, toki: String) {
	let translations = word.translations;
	let definition = translations
		.get(&toki)
		.map(|t| t.definition.clone())
		.unwrap_or_else(|| {
			format!(
				"No definition found for \"{}\" with language code \"{}\".",
				word.word, toki
			)
		});

	println!(
		"{} {} {}",
		"~>".bold(),
		word.word.bold(),
		if let Some(ucsur) = word.representations.and_then(|r| r.ucsur) {
			char::from_u32(u32::from_str_radix(ucsur.trim_start_matches("U+"), 16).unwrap())
				.map(|c| format!("({})", c))
				.unwrap()
		} else {
			String::new()
		}
	);

	println!(
		"{} {} {} - {}",
		colored_usage_category(&word.usage_category),
		format!("({}%) ·", get_usage_percentage(word.usage)).bright_black(),
		word.book.to_string().bright_black(),
		word.creator.join(", ")
	);

	println!("--------------");
	println!("{}", definition)
}

fn colored_usage_category(cat: &UsageCategory) -> ColoredString {
	match cat {
		UsageCategory::Core => "core".green(),
		UsageCategory::Common => "common".yellow(),
		UsageCategory::Uncommon => "uncommon".red(),
		UsageCategory::Obscure => "obscure".magenta(),
		UsageCategory::Sandbox => "sandbox".bright_black(),
	}
}

fn get_usage_percentage(usage: HashMap<String, u8>) -> u8 {
	let mut latest: (u16, u8) = (0, 0);
	for key in usage.keys() {
		let year = key[0..4].parse::<u16>().unwrap();
		let month = key[5..7].parse::<u8>().unwrap();
		if latest.0 < year || (latest.0 == year && latest.1 < month) {
			latest = (year, month);
		}
	}
	let key = format!("{}-{:02}", latest.0, latest.1);
	*usage.get(&key).unwrap()
}
