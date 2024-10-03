//! AIFF specific items

mod properties;
mod read;
pub(crate) mod tag;

use crate::id3::v2::tag::Id3v2Tag;

use moosicbox_lofty_attr::LoftyFile;

// Exports

pub use properties::{AiffCompressionType, AiffProperties};
pub use tag::{AIFFTextChunks, Comment};

/// An AIFF file
#[derive(LoftyFile)]
#[moosicbox_lofty(read_fn = "read::read_from")]
#[moosicbox_lofty(internal_write_module_do_not_use_anywhere_else)]
pub struct AiffFile {
	/// Any text chunks included in the file
	#[moosicbox_lofty(tag_type = "AiffText")]
	pub(crate) text_chunks_tag: Option<AIFFTextChunks>,
	/// An ID3v2 tag
	#[moosicbox_lofty(tag_type = "Id3v2")]
	pub(crate) id3v2_tag: Option<Id3v2Tag>,
	/// The file's audio properties
	pub(crate) properties: AiffProperties,
}
