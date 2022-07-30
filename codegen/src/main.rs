use askama::Template;
use serde::Deserialize;
use std::{
	collections::{BTreeMap, BTreeSet, HashMap},
	fs::{self, File},
	io::{self, Write},
	mem,
	path::Path
};

// from https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute#svg_attributes_by_category
fn attributes_by_category() -> BTreeMap<&'static str, BTreeSet<&'static str>> {
	let map: &[(&'static str, &'static [&'static str])] = &[
		("coreAttributes", &[
			"id",
			"lang",
			"tabindex",
			"xml:base",
			"xml:lang",
			"xml:space"
		]),
		("styleAttributes", &["class", "style"]),
		("conditionalProcessingAttributes", &[
			"requiredExtensions",
			"requiredFeatures",
			"systemLanguage"
		]),
		("xLinkAttributes", &[
			"xlink:href",
			"xlink:type",
			"xlink:role",
			"xlink:arcrole",
			"xlink:title",
			"xlink:show",
			"xlink:actuate"
		]),
		("presentationAttributes", &[
			"alignment-baseline",
			"baseline-shift",
			"clip",
			"clip-path",
			"clip-rule",
			"color",
			"color-interpolation",
			"color-interpolation-filters",
			"color-profile",
			"color-rendering",
			"cursor",
			"direction",
			"display",
			"dominant-baseline",
			"enable-background",
			"fill",
			"fill-opacity",
			"fill-rule",
			"filter",
			"flood-color",
			"flood-opacity",
			"font-family",
			"font-size",
			"font-size-adjust",
			"font-stretch",
			"font-style",
			"font-variant",
			"font-weight",
			"glyph-orientation-horizontal",
			"glyph-orientation-vertical",
			"image-rendering",
			"kerning",
			"letter-spacing",
			"lighting-color",
			"marker-end",
			"marker-mid",
			"marker-start",
			"mask",
			"opacity",
			"overflow",
			"pointer-events",
			"shape-rendering",
			"stop-color",
			"stop-opacity",
			"stroke",
			"stroke-dasharray",
			"stroke-dashoffset",
			"stroke-linecap",
			"stroke-linejoin",
			"stroke-miterlimit",
			"stroke-opacity",
			"stroke-width",
			"text-anchor",
			"text-decoration",
			"text-rendering",
			"transform",
			"transform-origin",
			"unicode-bidi",
			"vector-effect",
			"visibility",
			"word-spacing",
			"writing-mode"
		]),
		("filterAttributes", &["height", "result", "width", "x", "y"]),
		("transferFunctionAttributes", &[
			"type",
			"tableValues",
			"slope",
			"intercept",
			"amplitude",
			"exponent",
			"offset"
		]),
		// Animation target element attributes: &["href"]
		("animationAttributeTargetAttributes", &[
			"attributeType",
			"attributeName"
		]),
		("animationTimingAttributes", &[
			"begin",
			"dur",
			"end",
			"min",
			"max",
			"restart",
			"repeatCount",
			"repeatDur",
			"fill"
		]),
		("animationValueAttributes", &[
			"calcMode",
			"values",
			"keyTimes",
			"keySplines",
			"from",
			"to",
			"by",
			"autoReverse",
			"accelerate",
			"decelerate"
		]),
		("animationAdditionAttributes", &["additive", "accumulate"]),
		("animationEventAttributes", &[
			"onbegin", "onend", "onrepeat"
		]),
		("documentEventAttributes", &[
			"onabort", "onerror", "onresize", "onscroll", "onunload"
		]),
		("globalEventAttributes", &[
			"oncancel",
			"oncanplay",
			"oncanplaythrough",
			"onchange",
			"onclick",
			"onclose",
			"oncuechange",
			"ondblclick",
			"ondrag",
			"ondragend",
			"ondragenter",
			"ondragleave",
			"ondragover",
			"ondragstart",
			"ondrop",
			"ondurationchange",
			"onemptied",
			"onended",
			"onerror",
			"onfocus",
			"oninput",
			"oninvalid",
			"onkeydown",
			"onkeypress",
			"onkeyup",
			"onload",
			"onloadeddata",
			"onloadedmetadata",
			"onloadstart",
			"onmousedown",
			"onmouseenter",
			"onmouseleave",
			"onmousemove",
			"onmouseout",
			"onmouseover",
			"onmouseup",
			"onmousewheel",
			"onpause",
			"onplay",
			"onplaying",
			"onprogress",
			"onratechange",
			"onreset",
			"onresize",
			"onscroll",
			"onseeked",
			"onseeking",
			"onselect",
			"onshow",
			"onstalled",
			"onsubmit",
			"onsuspend",
			"ontimeupdate",
			"ontoggle",
			"onvolumechange",
			"onwaiting"
		]),
		("graphicalEventAttributes", &[
			"onactivate",
			"onfocusin",
			"onfocusout"
		])
	];
	map.iter()
		.map(|(key, value)| (*key, value.iter().copied().collect()))
		.collect()
}

#[derive(Deserialize)]
struct Data {
	elements: BTreeMap<String, Element>
}

#[derive(Deserialize)]
struct Element {
	categories: BTreeSet<String>,
	content: Content,
	attributes: BTreeSet<String>,
	#[serde(skip_deserializing, default)]
	attribute_categories: BTreeSet<String>
}

#[derive(Deserialize)]
struct Content {
	description: Description,
	#[serde(default)]
	elements: BTreeSet<String>
}

impl Content {
	fn is_cdata(&self) -> bool {
		self.description == "anyElementsOrCharacterData" && self.elements.is_empty()
	}
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

impl PartialEq<&str> for Description {
	fn eq(&self, other: &&str) -> bool {
		match self {
			Self::String(s) => s == other,
			_ => false
		}
	}
}

mod tpl {
	use super::Element;
	use askama::Template;
	use std::collections::{BTreeMap, BTreeSet};

	mod filters {
		use askama::Result;
		use heck::{ToSnakeCase, ToUpperCamelCase};

		pub fn camel_case(ident: &str) -> Result<String> {
			Ok(match ident {
				"g" => "Group".to_owned(),
				"tspan" => "TSpan".to_owned(),
				ident => ident.to_upper_camel_case()
			})
		}

		pub fn snake_case(ident: &str) -> Result<String> {
			Ok(match ident {
				"in" => "in1".to_owned(),
				"type" => "ty".to_owned(),
				ident => ident.to_snake_case()
			})
		}
	}

	#[derive(Template)]
	#[template(path = "common_attrs.rs.j2", escape = "none")]
	pub(super) struct CommonAttrs<'a> {
		pub(super) categories: &'a BTreeMap<&'static str, BTreeSet<&'static str>>,
		pub(super) empty_array: [String; 0]
	}

	#[derive(Template)]
	#[template(path = "tags.rs.j2", escape = "none")]
	pub(super) struct Tags<'a> {
		pub(super) elements: &'a BTreeMap<String, Element>
	}
}

fn write_tpl<T, P>(tpl: T, path: P) -> io::Result<()>
where
	T: Template,
	P: AsRef<Path>
{
	let tpl = tpl.render().unwrap();
	let mut file = File::create(path)?;
	writeln!(file, "{tpl}")?;
	Ok(())
}

fn main() {
	let file = File::open("SVGData.json").expect("Cannot find SVGData.json");
	let mut data: Data = serde_json::from_reader(file).expect("Cannot read SVGData.json");
	let attr_categories = attributes_by_category();

	let mut elem_categories: HashMap<String, Vec<String>> = HashMap::new();
	for (name, elem) in &data.elements {
		for cat in &elem.categories {
			let cat = format!("{cat}s");
			if let Some(v) = elem_categories.get_mut(&cat) {
				v.push(name.to_owned());
			} else {
				elem_categories.insert(cat, vec![name.to_owned()]);
			}
		}
	}
	for (name, elem) in data.elements.iter_mut() {
		println!("<{name}>: {:?}", elem.content.elements);
		let mut attributes = BTreeSet::new();
		let mut attribute_categories = BTreeSet::new();
		for attr in mem::take(&mut elem.attributes) {
			if attr.starts_with('\'') && attr.ends_with('\'') {
				attributes.insert(attr.trim_matches('\'').to_owned());
			} else {
				attribute_categories.insert(attr);
			}
		}
		if attribute_categories.contains("graphicalEventAttributes") {
			attribute_categories.insert("globalEventAttributes".to_owned());
		}
		elem.attributes = attributes;
		elem.attribute_categories = attribute_categories;

		let mut elements = BTreeSet::new();
		for elem in &elem.content.elements {
			if elem.starts_with("&lt;") && elem.ends_with("&gt;") {
				elements.insert(elem[4 .. elem.len() - 4].to_owned());
			} else if let Some(elems) = elem_categories.get(elem.as_str()) {
				elements.extend(elems.iter().cloned());
			} else {
				eprintln!(
					"ERROR: Missing category {elem}, I only know {:?}",
					elem_categories.keys().collect::<Vec<_>>()
				)
			}
		}
		elem.content.elements = elements;
	}

	let dir: &Path = "../src/tags".as_ref();
	fs::create_dir_all(dir).ok();
	write_tpl(
		tpl::Tags {
			elements: &data.elements
		},
		dir.join("mod.rs")
	)
	.expect("Failed to write tags/mod.rs");
	write_tpl(
		tpl::CommonAttrs {
			categories: &attr_categories,
			empty_array: []
		},
		dir.join("common_attrs.rs")
	)
	.expect("Failed to write tags/common_attrs.rs");
	for (name, elem) in &data.elements {
		File::create(dir.join(format!("{}.md", name)))
			.expect("Cannot create tag doc file")
			.write_all(elem.content.description.en_us().as_bytes())
			.expect("Cannot write tag doc file");
	}
}
