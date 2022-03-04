mod chunk_file;
mod frame;

use super::Id3v2TagFlags;
use crate::error::{ErrorKind, LoftyError, Result};
use crate::file::FileType;
use crate::id3::find_id3v2;
use crate::id3::v2::frame::FrameRef;
use crate::id3::v2::synch_u32;
use crate::id3::v2::tag::Id3v2TagRef;
use crate::probe::Probe;

use std::fs::File;
use std::io::{Cursor, Read, Seek, SeekFrom, Write};

use byteorder::{BigEndian, LittleEndian, WriteBytesExt};

#[allow(clippy::shadow_unrelated)]
pub(crate) fn write_id3v2<'a, I: Iterator<Item = FrameRef<'a>> + 'a>(
	data: &mut File,
	tag: &mut Id3v2TagRef<'a, I>,
) -> Result<()> {
	let probe = Probe::new(data).guess_file_type()?;
	let file_type = probe.file_type();

	let data = probe.into_inner();

	match file_type {
		Some(FileType::APE | FileType::MP3) => {},
		// Formats such as WAV and AIFF store the ID3v2 tag in an 'ID3 ' chunk rather than at the beginning of the file
		Some(FileType::WAV) => {
			tag.flags.footer = false;
			return chunk_file::write_to_chunk_file::<LittleEndian>(data, &create_tag(tag)?);
		},
		Some(FileType::AIFF) => {
			tag.flags.footer = false;
			return chunk_file::write_to_chunk_file::<BigEndian>(data, &create_tag(tag)?);
		},
		_ => return Err(LoftyError::new(ErrorKind::UnsupportedTag)),
	}

	let id3v2 = create_tag(tag)?;

	// find_id3v2 will seek us to the end of the tag
	find_id3v2(data, false)?;

	let mut file_bytes = Vec::new();
	data.read_to_end(&mut file_bytes)?;

	file_bytes.splice(0..0, id3v2);

	data.seek(SeekFrom::Start(0))?;
	data.set_len(0)?;
	data.write_all(&*file_bytes)?;

	Ok(())
}

pub(super) fn create_tag<'a, I: Iterator<Item = FrameRef<'a>> + 'a>(
	tag: &mut Id3v2TagRef<'a, I>,
) -> Result<Vec<u8>> {
	let frames = &mut tag.frames;
	let mut peek = frames.peekable();

	if peek.peek().is_none() {
		return Ok(Vec::new());
	}

	let has_footer = tag.flags.footer;
	let mut id3v2 = create_tag_header(tag.flags)?;
	let header_len = id3v2.get_ref().len();

	// Write the items
	frame::create_items(&mut id3v2, &mut peek)?;

	let len = id3v2.get_ref().len() - header_len;

	// Go back to the start and write the final size
	id3v2.seek(SeekFrom::Start(6))?;
	id3v2.write_u32::<BigEndian>(synch_u32(len as u32)?)?;

	if has_footer {
		id3v2.seek(SeekFrom::Start(3))?;

		let mut header_without_identifier = [0; 7];
		id3v2.read_exact(&mut header_without_identifier)?;
		id3v2.seek(SeekFrom::End(0))?;

		// The footer is the same as the header, but with the identifier reversed
		id3v2.write_all(b"3DI")?;
		id3v2.write_all(&header_without_identifier)?;
	}

	Ok(id3v2.into_inner())
}

fn create_tag_header(flags: Id3v2TagFlags) -> Result<Cursor<Vec<u8>>> {
	let mut header = Cursor::new(Vec::new());

	header.write_all(&[b'I', b'D', b'3'])?;

	let mut tag_flags = 0;

	// Version 4, rev 0
	header.write_all(&[4, 0])?;

	#[cfg(not(feature = "id3v2_restrictions"))]
	let extended_header = flags.crc;

	#[cfg(feature = "id3v2_restrictions")]
	let extended_header = flags.crc || flags.restrictions.0;

	if flags.footer {
		tag_flags |= 0x10
	}

	if flags.experimental {
		tag_flags |= 0x20
	}

	if extended_header {
		tag_flags |= 0x40
	}

	header.write_u8(tag_flags)?;
	header.write_u32::<BigEndian>(0)?;

	// TODO
	#[allow(unused_mut)]
	if extended_header {
		// Structure of extended header:
		//
		// Size (4)
		// Number of flag bytes (1) (As of ID3v2.4, this will *always* be 1)
		// Flags (1)
		// Followed by any extra data (crc or restrictions)

		// Start with a zeroed header
		header.write_all(&[0; 6])?;

		let mut size = 6_u32;
		let mut ext_flags = 0_u8;

		if flags.crc {
			// ext_flags |= 0x20;
			// size += 5;
			//
			// header.write_all(&[5, 0, 0, 0, 0, 0])?;
		}

		#[cfg(feature = "id3v2_restrictions")]
		if flags.restrictions.0 {
			ext_flags |= 0x10;
			size += 2;

			header.write_u8(1)?;
			header.write_u8(flags.restrictions.1.as_bytes())?;
		}

		header.seek(SeekFrom::Start(10))?;

		// Seek back and write the actual values
		header.write_u32::<BigEndian>(synch_u32(size)?)?;
		header.write_u8(1)?;
		header.write_u8(ext_flags)?;
	}

	Ok(header)
}
