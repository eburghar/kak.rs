//! Convert ansi-code to kakoune face

use crate::escape::{EscapeIterator, Mode, Token};

use yew_ansi::{get_sgr_segments, ColorEffect, SgrEffect};

/// Return the kakoune face representation in face
pub fn display_face(effect: &SgrEffect, face: &mut String) -> bool {
	let has_bg = effect.bg != ColorEffect::None;
	let has_fg = effect.fg != ColorEffect::None || has_bg;
	let has_option =
		effect.italic || effect.underline || effect.bold || effect.reverse || effect.dim;

	face.clear();

	if has_fg || has_bg || has_option {
		match effect.fg {
			ColorEffect::Name(color) => face.push_str(&format!("{}", color)),
			ColorEffect::NameBright(color) => face.push_str(&format!("bright-{}", color)),
			ColorEffect::Rgb(color) => face.push_str(&format!("rgb:{:X}", color)),
			ColorEffect::None => {
				if has_bg {
					face.push_str("default")
				}
			}
		};

		if has_fg && has_bg {
			face.push(',');
		}

		match effect.bg {
			ColorEffect::Name(color) => face.push_str(&format!("{}", color)),
			ColorEffect::NameBright(color) => face.push_str(&format!("bright-{}", color)),
			ColorEffect::Rgb(color) => face.push_str(&format!("rgb:{:06X}", color)),
			ColorEffect::None => (),
		};

		if has_option {
			face.push('+');
			if effect.italic {
				face.push('i');
			}
			if effect.underline {
				face.push('u');
			}
			if effect.bold {
				face.push('b');
			}
			if effect.reverse {
				face.push('r');
			}
			if effect.dim {
				face.push('d');
			}
		}
	}

	face != ""
}

/// print a text and replace ansi-code with kakoune faces {\[bgcolor\]\[,fgcolor\]\[+options\]}
pub fn print(ansi: &str, mode: Mode) {
	let mut face = String::with_capacity(64);
	for (effect, txt) in get_sgr_segments(ansi) {
		display_face(&effect, &mut face);
		print!("{{{}}}", &face);
		for token in EscapeIterator::new(txt, mode) {
			match token {
				Token::Str(txt) => print!("{}", txt), // all modes
				Token::Percent => print!("%%"), // block mode only
				Token::Block(txt) => print!("{}", txt), // block mode only
				Token::OpenBrace => print!("\\{{"), // brace mode only
			}
		}
	}
}
