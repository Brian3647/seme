use crate::error::Error;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, time::SystemTime};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
struct CacheEntry {
	content: String,
	created: SystemTime,
}

pub fn add_cache(url: String, result: String) -> Result<()> {
	let mut cache = read_cache_file()?;

	cache.insert(
		url,
		CacheEntry {
			content: result,
			created: SystemTime::now(),
		},
	);

	write_to_cache(cache)
}

#[allow(unused_must_use)]
pub fn get_from_cache(url: &String, cache_lifetime_seconds: u64) -> Result<Option<String>> {
	let mut cache = read_cache_file()?;
	if let Some(entry) = cache.get(url) {
		if SystemTime::now().duration_since(entry.created).unwrap()
			<= std::time::Duration::from_secs(cache_lifetime_seconds)
		{
			return Ok(Some(entry.content.clone()));
		}

		cache.remove(url);
		write_to_cache(cache);
	}

	Ok(None)
}

fn get_cachefile_path() -> std::path::PathBuf {
	let mut cachefile_path = dirs::cache_dir().unwrap();
	cachefile_path.push("seme.json");
	cachefile_path
}

fn read_cache_file() -> Result<HashMap<String, CacheEntry>> {
	let path = get_cachefile_path();
	if !path.exists() {
		fs::write(&path, "{}")?;
	}

	Ok(serde_json::from_str(&String::from_utf8(fs::read(path)?)?)?)
}

fn write_to_cache(cache: HashMap<String, CacheEntry>) -> Result<()> {
	Ok(fs::write(
		get_cachefile_path(),
		serde_json::to_string(&cache)?,
	)?)
}
