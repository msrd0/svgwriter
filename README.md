# svgwriter ![License](https://img.shields.io/crates/l/svgwriter) [![svgwriter on crates.io](https://img.shields.io/crates/v/svgwriter)](https://crates.io/crates/svgwriter) [![svgwriter on docs.rs](https://docs.rs/svgwriter/badge.svg)](https://docs.rs/svgwriter) [![Source Code Repository](https://img.shields.io/badge/Code-On%20Codeberg-blue?logo=Codeberg)](https://codeberg.org/msrd0/svgwriter)

`svgwriter` is a typed library for writing correct SVG files. It includes SVG
specification and documentation from [mdn][__link0].

### Example

```rust
use svgwriter::{
	tags::{Path, TagWithPresentationAttributes as _},
	Data, Graphic
};

let mut svg = Graphic::new();
let size = 100;
svg.set_width(size);
svg.set_height(size);
svg.set_view_box(format!("0 0 {size} {size}"));

// draw a heart, inspired by https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/d#example
let d = 40;
let padding = size / 2 - d;
let mut heart = Data::new();
heart
	.move_to(padding, padding + d / 2)
	.arc_by(d / 2, d / 2, 0, false, true, d, 0)
	.arc_by(d / 2, d / 2, 0, false, true, d, 0)
	.quad_by(0, d * 3 / 4, -d, d * 3 / 2)
	.quad_by(-d,-d * 3 / 4, -d, -d * 3 / 2);
svg.push(
	Path::new()
		.with_d(heart)
		.with_fill("#A919FA")
		.with_fill_opacity(0.5)
		.with_stroke("#A919FA")
		.with_stroke_width(3)
);

// write the svg to a file
write!(file, "{}", svg.to_string());
```

This code produces the following image:

<svg xmlns='http://www.w3.org/2000/svg' width='100' height='100' viewBox='0 0 100 100'><path d='M10,30a20,20,0,0,1,40,0a20,20,0,0,1,40,0q0,30,-40,60q-40,-30,-40,-60' fill='#A919FA' fill-opacity='0.5' stroke='#A919FA' stroke-width='3'/></svg>

![Image produced by example code](data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20width='100'%20height='100'%20viewBox='0%200%20100%20100'%3E%3Cpath%20d='M10,30a20,20,0,0,1,40,0a20,20,0,0,1,40,0q0,30,-40,60q-40,-30,-40,-60'%20fill='%23A919FA'%20fill-opacity='0.5'%20stroke='%23A919FA'%20stroke-width='3'/%3E%3C/svg%3E)


 [__link0]: https://developer.mozilla.org
