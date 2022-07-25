use askama::Template;
use serde::Deserialize;
use std::{
	collections::{BTreeMap, BTreeSet, HashMap},
	fs::{self, File},
	io::Write,
	path::Path
};

#[derive(Deserialize)]
struct Data {
	elements: BTreeMap<String, Element>
}

#[derive(Deserialize)]
struct Element {
	categories: BTreeSet<String>,
	content: Content,
	attributes: BTreeSet<String>
}

#[derive(Deserialize)]
struct Content {
	description: Description
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Description {
	String(String),
	Langs(HashMap<String, String>)
}

impl Description {
	fn en_us(&self) -> &str {
		match self {
			Self::String(descr) => descr,
			Self::Langs(langs) => &langs["en-US"]
		}
	}
}

mod tpl {
	use super::Element;
	use askama::Template;
	use std::collections::BTreeMap;

	mod filters {
		use askama::Result;
		use heck::{ToShoutySnakeCase, ToSnakeCase, ToUpperCamelCase};

		fn sanitize(ident: &str) -> String {
			ident.replace('+', "_plus_").replace('-', "_minus_")
		}

		pub fn camel_case(ident: &str) -> Result<String> {
			Ok(sanitize(ident).to_upper_camel_case())
		}

		pub fn snake_case(ident: &str) -> Result<String> {
			let sanitized = sanitize(ident);
			Ok(match sanitized.as_str() {
				"in" => "in1".to_owned(),
				"type" => "ty".to_owned(),
				sanitized => sanitized.to_snake_case()
			})
		}

		pub fn upper_case(ident: &str) -> Result<String> {
			Ok(sanitize(ident).to_shouty_snake_case())
		}
	}

	#[derive(Template)]
	#[template(path = "tags.rs.j2", escape = "none")]
	pub(super) struct Tags<'a> {
		pub(super) elements: &'a BTreeMap<String, Element>
	}
}

fn main() {
	let file = File::open("SVGData.json").expect("Cannot find SVGData.json");
	let mut data: Data = serde_json::from_reader(file).expect("Cannot read SVGData.json");

	for elem in data.elements.values_mut() {
		elem.attributes = elem
			.attributes
			.iter()
			.filter(|attr| attr.starts_with('\'') && attr.ends_with('\''))
			.map(|attr| attr.trim_matches('\'').to_owned())
			.collect();
	}

	let dir: &Path = "../src/tags".as_ref();
	fs::create_dir_all(dir).ok();
	let tags = tpl::Tags {
		elements: &data.elements
	}
	.render()
	.unwrap();
	File::create(dir.join("mod.rs"))
		.expect("Failed to create tags/mod.rs")
		.write_all(tags.as_bytes())
		.expect("Failed to write tags/mod.rs");
	for (name, elem) in &data.elements {
		File::create(dir.join(format!("{}.md", name)))
			.expect("Cannot create tag doc file")
			.write_all(elem.content.description.en_us().as_bytes())
			.expect("Cannot write tag doc file");
	}
}
