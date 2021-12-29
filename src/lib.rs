use std::time::Duration;

#[cfg(any(feature = "parse-names", feature = "require-parse-names"))]
/// Re-exported from [`torrent-name-parser`](torrent_name_parser::Metadata)
pub use torrent_name_parser::Metadata;

#[cfg(feature = "require-parse-names")]
/// Re-exported from [`torrent-name-parser`](torrent_name_parser::error::ErrorMatch)
pub use torrent_name_parser::error::ErrorMatch as ParseError;

#[derive(Clone, Debug, Default, PartialEq)]
/// Represents a torrent in a tracker somewhere.
pub struct Torrent {
	pub name: String,
	#[cfg(all(feature = "parse-names", not(feature = "require-parse-names")))]
	pub metadata: Option<Metadata>,
	#[cfg(feature = "require-parse-names")]
	pub metadata: Metadata,
	pub size: u64,
	pub categories: Vec<u32>,
	pub link: String,
	pub seeders: Option<u16>,
	pub leechers: Option<u16>,
	pub minimum_ratio: Option<f32>,
	pub minimum_seedtime: Option<Duration>
}

#[allow(clippy::too_many_arguments)]
impl Torrent {
	#[cfg(feature = "require-parse-names")]
	#[inline]
	pub fn new(name: String, size: u64, categories: Vec<u32>, link: String, seeders: Option<u16>, leechers: Option<u16>, minimum_ratio: Option<f32>, minimum_seedtime: Option<Duration>) -> Result<Self, torrent_name_parser::error::ErrorMatch> {
		Ok(Self{
			metadata: Metadata::from(&name)?,
			name,
			size,
			categories,
			link,
			seeders,
			leechers,
			minimum_ratio,
			minimum_seedtime
		})
	}

	#[cfg(not(feature = "require-parse-names"))]
	#[inline]
	/// Constructs a new `Torrent` from values.  If the `parse-names` or `require-parse-names` feature is enabled, will include metadata; if the `require-parse-names` feature is enabled, parsing is non-optional, and the function will return an error.
	pub fn new(name: String, size: u64, categories: Vec<u32>, link: String, seeders: Option<u16>, leechers: Option<u16>, minimum_ratio: Option<f32>, minimum_seedtime: Option<Duration>) -> Self {
		Self{
			#[cfg(feature = "parse-names")]
			metadata: Metadata::from(&name).ok(),
			name,
			size,
			categories,
			link,
			seeders,
			leechers,
			minimum_ratio,
			minimum_seedtime
		}
	}
}

