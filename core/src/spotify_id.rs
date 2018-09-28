use byteorder::{BigEndian, ByteOrder};
use extprim::u128::u128;
use std;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpotifyTrackType {
    Track,
    Podcast,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpotifyId {
    pub id: u128,
    pub track_type: SpotifyTrackType,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct SpotifyIdError;

const BASE62_DIGITS: &'static [u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const BASE16_DIGITS: &'static [u8] = b"0123456789abcdef";

impl SpotifyId {
    fn as_track(n: u128) -> SpotifyId {
        SpotifyId {
            id: n.to_owned(),
            track_type: SpotifyTrackType::Track,
        }
    }

    pub fn from_base16(id: &str) -> Result<SpotifyId, SpotifyIdError> {
        let data = id.as_bytes();

        let mut n: u128 = u128::zero();
        for c in data {
            let d = match BASE16_DIGITS.iter().position(|e| e == c) {
                None => return Err(SpotifyIdError),
                Some(x) => x as u64,
            };
            n = n * u128::new(16);
            n = n + u128::new(d);
        }

        Ok(SpotifyId::as_track(n))
    }

    pub fn from_base62(id: &str) -> Result<SpotifyId, SpotifyIdError> {
        let data = id.as_bytes();

        let mut n: u128 = u128::zero();
        for c in data {
            let d = match BASE62_DIGITS.iter().position(|e| e == c) {
                None => return Err(SpotifyIdError),
                Some(x) => x as u64,
            };
            n = n * u128::new(62);
            n = n + u128::new(d);
        }
        Ok(SpotifyId::as_track(n))
    }

    pub fn from_raw(data: &[u8]) -> Result<SpotifyId, SpotifyIdError> {
        if data.len() != 16 {
            return Err(SpotifyIdError);
        };

        let high = BigEndian::read_u64(&data[0..8]);
        let low = BigEndian::read_u64(&data[8..16]);

        Ok(SpotifyId::as_track(u128::from_parts(high, low)))
    }

    pub fn from_uri(uri: &str) -> Result<SpotifyId, SpotifyIdError> {
        let parts = uri.split(":").collect::<Vec<&str>>();
        if uri.contains(":show:") || uri.contains(":episode:") {
            let mut spotify_id = SpotifyId::from_base62(parts[2]).unwrap();
            spotify_id.track_type = SpotifyTrackType::Podcast;
            Ok(spotify_id)
        } else {
            SpotifyId::from_base62(parts[2])
        }
    }

    pub fn to_base16(&self) -> String {
        let &SpotifyId { id: ref n, .. } = self;

        let mut data = [0u8; 32];
        for i in 0..32 {
            data[31 - i] = BASE16_DIGITS[(n.wrapping_shr(4 * i as u32).low64() & 0xF) as usize];
        }

        std::str::from_utf8(&data).unwrap().to_owned()
    }

    pub fn to_base62(&self) -> String {
        let &SpotifyId { id: mut n, .. } = self;

        let mut data = [0u8; 22];
        let sixty_two = u128::new(62);
        for i in 0..22 {
            data[21 - i] = BASE62_DIGITS[(n % sixty_two).low64() as usize];
            n /= sixty_two;
        }

        std::str::from_utf8(&data).unwrap().to_owned()
    }

    pub fn to_raw(&self) -> [u8; 16] {
        let &SpotifyId { id: ref n, .. } = self;

        let mut data = [0u8; 16];

        BigEndian::write_u64(&mut data[0..8], n.high64());
        BigEndian::write_u64(&mut data[8..16], n.low64());

        data
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileId(pub [u8; 20]);

impl FileId {
    pub fn to_base16(&self) -> String {
        self.0
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<String>>()
            .concat()
    }
}

impl fmt::Debug for FileId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("FileId").field(&self.to_base16()).finish()
    }
}

impl fmt::Display for FileId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.to_base16())
    }
}
