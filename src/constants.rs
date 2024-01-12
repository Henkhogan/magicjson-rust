pub const QUOTE_CHARS: [u8; 2] = [
    0x22, // "
    0x27, // '
    ];

pub const WHITESPACE_CHARS: [u8; 4] = [
    0x20, // ' '
    0x0A, // '\n'
    0x09, // '\t'
    0x0D, // '\r'
    ];

pub const AFTER_NULL_CHARS: [u8; 3] = [
    0x7D, // }
    0x5D, // ]
    0x2C, // ,
    ];

pub const COLON_CHAR: u8 = 0x3A; // :
pub const COMMA_CHAR: u8 = 0x2C; // ,
pub const DICT_START_CHAR: u8 = 0x7B; // {
pub const DICT_END_CHAR: u8 = 0x7D; // }
pub const DOT_CHAR: u8 = 0x2E; // .
pub const ESCAPE_CHAR: u8 = 0x5C; // \
pub const LIST_START_CHAR: u8 = 0x5B; // [
pub const LIST_END_CHAR: u8 = 0x5D; // ]
pub const LOOP_MAX_ITERATIONS: u16 = 100;
pub const MAX_ITEMS: u16 = 100;

pub const MINUS_CHAR: u8 = 0x2D; // -
pub const PLUS_CHAR: u8 = 0x2B; // +

pub const DIGIT_CHARS: [u8; 10] = [
    0x30, // 0
    0x31, // 1
    0x32, // 2
    0x33, // 3
    0x34, // 4
    0x35, // 5
    0x36, // 6
    0x37, // 7
    0x38, // 8
    0x39, // 9
    ];

pub const NUMERIC_CHARS: [u8; 11] = [
    0x2E, // .
    0x30, // 0
    0x31, // 1
    0x32, // 2
    0x33, // 3
    0x34, // 4
    0x35, // 5
    0x36, // 6
    0x37, // 7
    0x38, // 8
    0x39, // 9
    ];
