use std::env::args;

use colored::ColoredString;
use colored::Colorize;
use isahc::ReadResponseExt;
use itertools::Itertools;
use linku_sona::{UsageCategory, Word};

mod error;

#[derive(serde::Deserialize)]
#[serde(untagged)]
pub enum ApiResult {
	Word(Box<Word>),
	Error { message: String },
}

fn main() -> Result<(), error::Error> {
	let mut args = args().skip(1);

	let word = args.next().unwrap_or("toki".into());
	let url = format!("https://api.linku.la/v1/words/{}", word);
	let response_string = isahc::get(url)?.text()?;

	if let Some("--json") | Some("-j") = args.next().as_deref() {
		println!("{}", response_string);
		return Ok(());
	}

	let response: ApiResult = serde_json::from_str(&response_string)?;
	match response {
		ApiResult::Word(word) => show(word),
		ApiResult::Error { message } => eprintln!("Error: {}", message),
	}

	Ok(())
}

fn show(word: Box<Word>) {
	let definition = if let Some(pu_data) = word.pu_verbatim {
		pu_data.en
	} else if let Some(ku_data) = word.ku_data {
		ku_data.keys().join(", ")
	} else {
		word.translations.get("en").unwrap().definition.clone()
	};

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
		"{} · {} - {}",
		colored_usage_category(&word.usage_category),
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
