pub mod authority;
pub mod path;
pub mod scheme;
pub mod uri;

pub use authority::Host;
pub use authority::Port;
pub use path::Path;
pub use scheme::Scheme;

macro_rules! char_colon {
    () => {
        0x3a
    };
}
macro_rules! char_slash {
    () => {
        0x2f
    };
}
