#[macro_export]
macro_rules! is_alpha {
    ($char: expr) => {
        (($char >= 0x41 && $char <= 0x5a) || ($char >= 0x61 && $char <= 0x7a))
    };
}

#[macro_export]
macro_rules! is_digit {
    ($char: expr) => {
        ($char >= 0x30 && $char <= 0x39)
    };
}

#[macro_export]
macro_rules! is_unreserved {
    ($char: expr) => {
        ($crate::is_alpha!($char)
            || $crate::is_digit!($char)
            || $char == 0x2d
            || $char == 0x2e
            || $char == 0x5f
            || $char == 0x7e)
    };
}

#[macro_export]
macro_rules! is_hexdig {
    ($char: expr) => {
        ($crate::is_digit!($char)
            || ($char >= 0x41 && $char <= 0x46)
            || ($char >= 0x61 && $char <= 0x66))
    };
}

#[macro_export]
macro_rules! is_sub_delims {
    ($char: expr) => {
        ($char == 0x21
            || $char == 0x24
            || ($char >= 0x26 && $char <= 0x2c)
            || $char == 0x3b
            || $char == 0x3d)
    };
}
