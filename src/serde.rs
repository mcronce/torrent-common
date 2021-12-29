#[cfg(feature = "require-parse-names")]
use std::convert::TryFrom;

use std::time::Duration;

#[derive(Clone, Debug, serde::Deserialize)]
/// Helper struct that derives Deserialize, which can then be converted to [`Torrent`](crate::Torrent) with `.into()` or `.try_into()`
pub struct Torrent {
	pub name: String,
	pub size: u64,
	pub categories: Vec<u32>,
	pub link: String,
	pub seeders: Option<u16>,
	pub leechers: Option<u16>,
	pub minimum_ratio: Option<f32>,
	pub minimum_seedtime: Option<Duration>
}

#[cfg(feature = "require-parse-names")]
impl TryFrom<Torrent> for crate::Torrent {
	type Error = torrent_name_parser::error::ErrorMatch;
	fn try_from(this: Torrent) -> Result<Self, Self::Error> {
		crate::Torrent::new(
			this.name,
			this.size,
			this.categories,
			this.link,
			this.seeders,
			this.leechers,
			this.minimum_ratio,
			this.minimum_seedtime
		)
	}
}

#[cfg(not(feature = "require-parse-names"))]
impl From<Torrent> for crate::Torrent {
	fn from(this: Torrent) -> Self {
		crate::Torrent::new(
			this.name,
			this.size,
			this.categories,
			this.link,
			this.seeders,
			this.leechers,
			this.minimum_ratio,
			this.minimum_seedtime
		)
	}
}

