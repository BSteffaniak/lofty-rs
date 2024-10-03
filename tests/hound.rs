use std::fs;
use std::fs::File;
use std::path::Path;

use hound::WavReader;
use moosicbox_lofty::iff::wav::WavFile;
use moosicbox_lofty::{AudioFile, ParseOptions, Result};

fn get_properties(path: &Path) -> Result<<WavFile as AudioFile>::Properties> {
	let mut f = File::open(path).unwrap();
	let wav_file = WavFile::read_from(&mut f, ParseOptions::new())?;
	Ok(*wav_file.properties())
}

#[test]
fn hound() {
	let paths = fs::read_dir("tests/files/assets/hound").unwrap();

	for path in paths {
		let path = path.unwrap().path();
		if path.is_file() && path.extension().unwrap() == "wav" {
			println!("Name: {}", path.display());
			let wav_reader = WavReader::open(&path).unwrap();
			let moosicbox_lofty = get_properties(&path).unwrap();
			assert_eq!(moosicbox_lofty.channels() as u16, wav_reader.spec().channels);
			assert_eq!(moosicbox_lofty.sample_rate(), wav_reader.spec().sample_rate);
			assert_eq!(moosicbox_lofty.bit_depth() as u16, wav_reader.spec().bits_per_sample);
		}
	}
}

#[test]
fn hound_fuzz() {
	let paths = fs::read_dir("tests/files/assets/hound/fuzz").unwrap();

	for path in paths {
		let path = path.unwrap().path();
		if path.is_file() && path.extension().unwrap() == "wav" {
			println!("Name: {}", path.display());
			if let Ok(wav_reader) = WavReader::open(&path) {
				let moosicbox_lofty = get_properties(&path).unwrap();
				println!("{moosicbox_lofty:#?}");
				assert_eq!(moosicbox_lofty.channels() as u16, wav_reader.spec().channels);
				assert_eq!(moosicbox_lofty.sample_rate(), wav_reader.spec().sample_rate);
				assert_eq!(moosicbox_lofty.bit_depth() as u16, wav_reader.spec().bits_per_sample);
			} else if get_properties(&path).is_ok() {
				println!("We are even better for this file!");
			}
		}
	}
}
