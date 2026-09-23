use bytes::{BufMut, Bytes, BytesMut};

use crate::grammar::{
    find_query_or_fragment, parse_path_abempty, parse_path_absolute, parse_path_noscheme,
    parse_path_rootless,
};
use crate::UriError;

use self::authority::{authority_wire_len, parse_authority, validate_host, write_authority};

mod authority;
mod path;

pub use authority::{Authority, Host, Port, Userinfo};
pub use path::Path;

#[derive(Debug, Clone, PartialEq)]
pub struct HierPart {
    pub origin: Bytes,
    pub authority: Option<Authority>,
    pub path: Path,
}

impl HierPart {
    #[inline]
    pub fn bytes(&self) -> Bytes {
        self.origin.clone()
    }

    #[inline]
    pub fn from_bytes(input: Bytes) -> Result<Self, UriError> {
        let mut cursor = 0;
        let hier = parse_hier(&input, &mut cursor, input.len(), true)?;
        if cursor != input.len() {
            return Err(UriError::InvalidUri);
        }
        Ok(hier)
    }

    #[inline]
    pub fn from_slice(input: &[u8]) -> Result<Self, UriError> {
        Self::from_bytes(Bytes::copy_from_slice(input))
    }

    pub fn parse(input: &Bytes, start: &mut usize, end: usize) -> Result<Self, UriError> {
        parse_hier(input, start, end, true)
    }
}

pub(crate) fn parse_hier(
    input: &Bytes,
    cursor: &mut usize,
    end: usize,
    has_scheme: bool,
) -> Result<HierPart, UriError> {
    if *cursor > end || end > input.len() {
        return Err(UriError::InvalidUri);
    }
    let bytes = input.as_ref();
    let start = *cursor;
    let limit = find_query_or_fragment(bytes, start, end);

    if start + 1 < limit && bytes[start] == b'/' && bytes[start + 1] == b'/' {
        let auth_start = start + 2;
        let mut auth_end = auth_start;
        while auth_end < limit && bytes[auth_end] != b'/' {
            auth_end += 1;
        }
        let authority = parse_authority(input, auth_start, auth_end)?;
        let path_end = parse_path_abempty(bytes, auth_end, limit)?;
        if path_end != limit {
            return Err(UriError::InvalidPath);
        }
        *cursor = limit;
        return Ok(HierPart {
            origin: input.slice(start..limit),
            authority: Some(authority),
            path: Path {
                origin: input.slice(auth_end..limit),
            },
        });
    }

    if start < limit && bytes[start] == b'/' {
        let path_end = parse_path_absolute(bytes, start, limit)?;
        if path_end != limit {
            return Err(UriError::InvalidPath);
        }
        *cursor = limit;
        let path = input.slice(start..limit);
        return Ok(HierPart {
            origin: path.clone(),
            authority: None,
            path: Path { origin: path },
        });
    }

    if start < limit {
        let path_end = if has_scheme {
            parse_path_rootless(bytes, start, limit)?
        } else {
            parse_path_noscheme(bytes, start, limit)?
        };
        if path_end != limit {
            return Err(UriError::InvalidPath);
        }
        *cursor = limit;
        let path = input.slice(start..limit);
        return Ok(HierPart {
            origin: path.clone(),
            authority: None,
            path: Path { origin: path },
        });
    }

    let empty = input.slice(start..start);
    Ok(HierPart {
        origin: empty.clone(),
        authority: None,
        path: Path { origin: empty },
    })
}

pub(crate) fn validate_hier(hier: &HierPart, has_scheme: bool) -> Result<(), UriError> {
    if let Some(authority) = &hier.authority {
        if let Some(userinfo) = &authority.userinfo {
            crate::grammar::validate_userinfo(&userinfo.origin)?;
        }
        validate_host(&authority.host)?;
        if let Some(port) = &authority.port {
            crate::grammar::validate_port(&port.origin)?;
        }
        let path = hier.path.origin.as_ref();
        if path.is_empty() {
            return Ok(());
        }
        if !path.starts_with(b"/") {
            return Err(UriError::InvalidPath);
        }
        let end = parse_path_abempty(path, 0, path.len())?;
        return if end == path.len() {
            Ok(())
        } else {
            Err(UriError::InvalidPath)
        };
    }

    let path = hier.path.origin.as_ref();
    if path.is_empty() {
        return Ok(());
    }
    if path.starts_with(b"//") {
        return Err(UriError::InvalidPath);
    }
    let end = if path.starts_with(b"/") {
        parse_path_absolute(path, 0, path.len())?
    } else if has_scheme {
        parse_path_rootless(path, 0, path.len())?
    } else {
        parse_path_noscheme(path, 0, path.len())?
    };
    if end == path.len() {
        Ok(())
    } else {
        Err(UriError::InvalidPath)
    }
}

pub(crate) fn hier_wire_len(hier: &HierPart) -> usize {
    let mut len = hier.path.origin.len();
    if let Some(authority) = &hier.authority {
        len += 2 + authority_wire_len(authority);
    }
    len
}

pub(crate) fn write_hier(hier: &HierPart, out: &mut BytesMut) {
    if let Some(authority) = &hier.authority {
        out.put_slice(b"//");
        write_authority(authority, out);
    }
    out.put_slice(&hier.path.origin);
}

#[derive(Debug, Clone, PartialEq)]
pub struct HierPartBuilder {
    pub authority: Option<Authority>,
    pub path: Option<Path>,
}

impl HierPartBuilder {
    #[inline]
    pub fn new() -> Self {
        Self {
            authority: None,
            path: None,
        }
    }

    pub fn authority(&mut self, authority: Authority) -> &mut Self {
        self.authority = Some(authority);
        self
    }

    pub fn path(&mut self, path: Path) -> &mut Self {
        self.path = Some(path);
        self
    }

    pub fn build(&self) -> Result<HierPart, UriError> {
        let path = match &self.path {
            Some(path) => path.clone(),
            None => Path {
                origin: Bytes::new(),
            },
        };
        let hier = HierPart {
            origin: Bytes::new(),
            authority: self.authority.clone(),
            path,
        };
        validate_hier(&hier, true)?;
        let len = hier_wire_len(&hier);
        let mut out = BytesMut::with_capacity(len);
        write_hier(&hier, &mut out);
        debug_assert_eq!(out.len(), len);
        let bytes = out.freeze();
        let mut cursor = 0;
        parse_hier(&bytes, &mut cursor, bytes.len(), true)
    }
}
