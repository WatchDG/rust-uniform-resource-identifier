/// Set of bytes that may appear unescaped in a URI component.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EncodeSet([u64; 4]);

impl EncodeSet {
    #[inline]
    pub const fn contains(self, byte: u8) -> bool {
        let idx = (byte >> 6) as usize;
        let bit = byte & 63;
        (self.0[idx] & (1u64 << bit)) != 0
    }

    pub(crate) const fn union(self, other: Self) -> Self {
        EncodeSet([
            self.0[0] | other.0[0],
            self.0[1] | other.0[1],
            self.0[2] | other.0[2],
            self.0[3] | other.0[3],
        ])
    }

    const fn from_bytes(bytes: &[u8]) -> Self {
        let mut set = [0u64; 4];
        let mut i = 0;
        while i < bytes.len() {
            let byte = bytes[i];
            let idx = (byte >> 6) as usize;
            let bit = byte & 63;
            set[idx] |= 1u64 << bit;
            i += 1;
        }
        EncodeSet(set)
    }

    const fn range(start: u8, end: u8) -> Self {
        let mut set = [0u64; 4];
        let mut byte = start;
        loop {
            let idx = (byte >> 6) as usize;
            let bit = byte & 63;
            set[idx] |= 1u64 << bit;
            if byte == end {
                break;
            }
            byte += 1;
        }
        EncodeSet(set)
    }

    pub(crate) const ALPHA: Self = Self::range(b'A', b'Z').union(Self::range(b'a', b'z'));
    pub(crate) const DIGIT: Self = Self::range(b'0', b'9');
    pub(crate) const HEXDIG: Self = Self::DIGIT
        .union(Self::range(b'A', b'F'))
        .union(Self::range(b'a', b'f'));
    pub(crate) const SCHEME: Self = Self::ALPHA
        .union(Self::DIGIT)
        .union(Self::from_bytes(b"+-."));
    pub(crate) const UNRESERVED: Self = Self::ALPHA
        .union(Self::DIGIT)
        .union(Self::from_bytes(b"-._~"));
    pub(crate) const SUB_DELIMS: Self = Self::from_bytes(b"!$&'()*+,;=");
    pub(crate) const IPV_FUTURE_TAIL: Self = Self::UNRESERVED
        .union(Self::SUB_DELIMS)
        .union(Self::from_bytes(b":"));

    /// `unreserved / sub-delims / ":"`
    pub const USERINFO: Self = Self::UNRESERVED
        .union(Self::SUB_DELIMS)
        .union(Self::from_bytes(b":"));
    /// `unreserved / sub-delims`
    pub const REG_NAME: Self = Self::UNRESERVED.union(Self::SUB_DELIMS);
    /// `unreserved / sub-delims / ":" / "@"`
    pub const SEGMENT: Self = Self::UNRESERVED
        .union(Self::SUB_DELIMS)
        .union(Self::from_bytes(b":@"));
    /// First segment of `path-noscheme`: no colon.
    pub(crate) const SEGMENT_NC: Self = Self::UNRESERVED
        .union(Self::SUB_DELIMS)
        .union(Self::from_bytes(b"@"));
    /// Segment bytes plus `"/"`.
    pub const PATH: Self = Self::SEGMENT.union(Self::from_bytes(b"/"));
    /// `pchar / "/" / "?"`
    pub const QUERY: Self = Self::SEGMENT.union(Self::from_bytes(b"/?"));
    /// Key or value of a query pair. Only `unreserved` stays literal.
    pub const QUERY_PAIR: Self = Self::UNRESERVED;
    /// `pchar / "/" / "?"`
    pub const FRAGMENT: Self = Self::QUERY;
}
