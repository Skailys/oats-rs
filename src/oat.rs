use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::{DecodeError, Engine};
use std::hash::{Hash, Hasher};
use std::num::ParseIntError;

/// A struct that represents an Oat.
pub struct Oat {
    /// The node for the Oat.
    node: u8,
    /// The locally unique identifier for the Oat.
    luid: u64,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ParseOatError {
    InvalidNode(ParseIntError),
    InvalidLUIDForm(DecodeError),
    InvalidLUIDLength(usize),
}

impl Oat {
    /// Creates a new Oat with the given node, sequence number, and timestamp.
    ///
    /// # Arguments
    ///
    /// * `node` - The node identifier.
    /// * `seq` - The sequence number.
    /// * `timestamp` - The timestamp.
    ///
    /// # Assertions
    ///
    /// The `ctlz` function is called on `seq` and `timestamp` with the respective bit lengths.
    /// Both assertions will fail if the result of `ctlz` is not greater than or equal to the bit length minus 12.
    ///
    /// # Examples
    ///
    /// ```
    /// use oats::oat::Oat;
    ///
    /// let oat = Oat::of(1, 0xfff, 0xfffffffffff);
    /// ```
    pub fn of(node: u8, seq: u16, timestamp: u64) -> Self {
        assert!(seq < (1 << 12));
        assert!(timestamp < (1 << 44));

        let luid = (timestamp << 12) | seq as u64;

        Oat { node, luid }
    }

    pub fn to_bytes(&self) -> [u8; 9] {
        let mut bytes = [0; 9];
        bytes[0] = self.node;
        bytes[1..].copy_from_slice(&self.luid.to_le_bytes());
        bytes
    }

    /// Parses a string representation of an Oat and returns a new Oat instance.
    ///
    /// # Arguments
    ///
    /// * `string` - The string representation of the Oat.
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed Oat if successful, or a `ParseOatError` if parsing fails.
    ///
    /// # Examples
    ///
    /// ```
    /// use oats::oat::Oat;
    ///
    /// let oat = Oat::from_string("X1AwCIGvFTGAA").expect("Failed to parse Oat.");
    /// assert_eq!(oat.node(), 1);
    /// assert_eq!(oat.seq(), 3);
    /// assert_eq!(oat.timestamp(), 1671800400_000);
    /// ```
    pub fn from_string(string: &str) -> Result<Self, ParseOatError> {
        let node = u8::from_str_radix(&string[0..2].replace('X', "0"), 16)
            .map_err(ParseOatError::InvalidNode)?;
        let luid = URL_SAFE_NO_PAD.decode(&string[2..]).map_err(ParseOatError::InvalidLUIDForm)?;

        if luid.len() != 8 {
            return Err(ParseOatError::InvalidLUIDLength(luid.len()));
        }

        let mut luid_bytes = [0; 8];
        luid_bytes.copy_from_slice(&luid);

        Ok(Oat {
            node,
            luid: u64::from_le_bytes(luid_bytes),
        })
    }

    /// Parses a string representation of an Oat and returns a new Oat instance without performing any checks.
    ///
    /// # Arguments
    ///
    /// * `string` - The string representation of the Oat.
    ///
    /// # Returns
    ///
    /// A new Oat instance created from the string representation without any checks.
    ///
    /// # Safety
    ///
    /// This function assumes that the input string is a valid representation of an Oat and does not perform any validation or error handling.
    /// Incorrect input may lead to undefined behavior.
    pub fn from_string_unchecked(string: &str) -> Self {
        let node = u8::from_str_radix(&string[0..2].replace('X', "0"), 16).unwrap();
        let luid = URL_SAFE_NO_PAD.decode(&string[2..]).unwrap();

        let mut luid_bytes = [0; 8];
        luid_bytes.copy_from_slice(&luid);

        Oat {
            node,
            luid: u64::from_le_bytes(luid_bytes),
        }
    }

    /// Creates a new Oat from a byte array.
    ///
    /// # Arguments
    ///
    /// * `bytes` - The byte array representing the Oat.
    ///
    /// # Returns
    ///
    /// A new Oat instance created from the byte array.
    ///
    /// # Panics
    ///
    /// This function will panic if the length of the byte array is not 9.
    ///
    /// # Examples
    ///
    /// ```
    /// use oats::oat::Oat;
    ///
    /// let bytes = [1, 0x03, 0x20, 0x88, 0x1A, 0xF1, 0x53, 0x18, 0x20];
    /// let oat = Oat::from_bytes(bytes);
    /// ```
    pub fn from_bytes(bytes: [u8; 9]) -> Self {
        Oat {
            node: bytes[0],
            luid: u64::from_le_bytes(bytes[1..].try_into().unwrap()),
        }
    }

    pub fn from_bytes_ref(bytes: &[u8]) -> Result<Self, ParseOatError> {
        if bytes.len() != 9 {
            return Err(ParseOatError::InvalidLUIDLength(bytes.len()));
        }

        Ok(Oat {
            node: bytes[0],
            luid: u64::from_le_bytes(bytes[1..].try_into().unwrap()),
        })
    }
}

impl Oat {
    /// Returns the node for the Oat.
    ///
    /// # Examples
    ///
    /// ```
    /// use oats::oat::Oat;
    ///
    /// let oat = Oat::of(1, 0, 0);
    /// assert_eq!(oat.node(), 1);
    /// ```
    pub fn node(&self) -> u8 {
        self.node
    }

    /// Returns the sequence number for the Oat.
    ///
    /// # Examples
    ///
    /// ```
    /// use oats::oat::Oat;
    ///
    /// let oat = Oat::of(1, 0xfff, 0);
    /// assert_eq!(oat.seq(), 0xfff);
    /// ```
    pub fn seq(&self) -> u16 {
        (self.luid & 0xfff) as u16
    }

    /// Returns the timestamp for the Oat.
    ///
    /// # Examples
    ///
    /// ```
    /// use oats::oat::Oat;
    ///
    /// let oat = Oat::of(1, 0, 0xfffffffffff);
    /// assert_eq!(oat.timestamp(), 0xfffffffffff);
    /// ```
    pub fn timestamp(&self) -> u64 {
        self.luid >> 12
    }

    /// Hashes the Oat using the given `Hasher` implementation and returns the result as a `String`.
    ///
    /// WARNING: The provided hash function might not be cryptographically secure.
    /// Because the returned hash is just a u64, collisions are not unlikely (https://en.wikipedia.org/wiki/Birthday_attack).
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::hash_map::RandomState;
    /// use std::hash::{BuildHasher, Hasher};
    /// use oats::oat::Oat;
    ///
    /// let oat = Oat::of(1, 0, 0);
    /// let hasher = RandomState::new().build_hasher();
    /// let hash = oat.hashed(hasher);
    /// ```
    pub fn hashed<H: Hasher>(&self, mut new: H) -> String {
        self.hash(&mut new);
        let hash = new.finish();

        format!(
            "{:X>2X}{}",
            self.node,
            URL_SAFE_NO_PAD.encode(&hash.to_le_bytes())
        )
    }
}

impl Hash for Oat {
    /// Hashes the Oat using the given `Hasher` implementation.
    ///
    /// # Examples
    ///
    /// ```
    /// use oats::oat::Oat;
    /// use std::collections::hash_map::RandomState;
    /// use std::hash::{BuildHasher, Hash, Hasher};
    ///
    /// let oat = Oat::of(1, 0, 0);
    /// let mut hasher = RandomState::new().build_hasher();
    /// oat.hash(&mut hasher);
    /// ```
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Write the locally unique identifier to the hasher.
        state.write(&self.luid.to_le_bytes())
    }
}

impl ToString for Oat {
    /// Converts the Oat to a string representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use oats::oat::Oat;
    ///
    /// let oat = Oat::of(1, 0, 0);
    /// let string = oat.to_string();
    /// ```
    fn to_string(&self) -> String {
        // Format the locally unique identifier and node as a string.
        format!(
            "{:X>2X}{}",
            &self.node,
            URL_SAFE_NO_PAD.encode(&self.luid.to_le_bytes())
        )
    }
}

/// Implements the conversion of `Oat` into a `String`.
impl Into<String> for Oat {
    /// Converts the `Oat` into a `String`.
    ///
    /// # Examples
    ///
    /// ```
    /// use oats::oat::Oat;
    ///
    /// let oat = Oat::of(1, 3, 1671800400_000);
    /// let string: String = oat.into();
    /// assert_eq!(string, "X1AwCIGvFTGAA");
    /// ```
    fn into(self) -> String {
        self.to_string()
    }
}

/// Converts an `Oat` struct into a fixed-size byte array of length 9.
impl Into<[u8; 9]> for Oat {
    /// Converts the `Oat` struct into a byte array.
    ///
    /// # Returns
    ///
    /// A fixed-size byte array of length 9.
    ///
    /// # Examples
    ///
    /// ```
    /// use oats::oat::Oat;
    ///
    /// let oat = Oat::of(1, 3, 1671800400_000);
    /// let bytes: [u8; 9] = oat.into();
    /// 
    /// assert_eq!(bytes, [1, 0x03, 0x0, 0x88, 0x1A, 0xF1, 0x53, 0x18, 0x0]);
    /// ```
    fn into(self) -> [u8; 9] {
        let mut bytes = [0; 9];
        bytes[0] = self.node;
        bytes[1..].copy_from_slice(&self.luid.to_le_bytes());
        bytes
    }
}
