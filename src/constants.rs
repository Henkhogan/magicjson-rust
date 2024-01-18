
use std::collections::BTreeSet;

lazy_static! {
    pub static ref QUOTE_CHARS: BTreeSet<u8> = {
        let mut m = BTreeSet::new();
        m.insert(0x22);
        m.insert(0x27);
        m
    };
}

//pub const QUOTE_CHARS: BTreeSet<u8> = BTreeSet::from_iter([ 
//    0x22, // "
//    0x27, // '
//    ]);

lazy_static! {
    pub static ref WHITESPACE_CHARS: BTreeSet<u8> = {
        let mut m = BTreeSet::new();
        m.insert(0x20);
        m.insert(0x0A);
        m.insert(0x09);
        m.insert(0x0D);
        m
    };
}

//pub const WHITESPACE_CHARS: [u8; 4] = [
//    0x20, // ' '
//    0x0A, // '\n'
//    0x09, // '\t'
//    0x0D, // '\r'
//    ];


lazy_static! {
    pub static ref AFTER_NULL_CHARS: BTreeSet<u8> = {
        let mut m = BTreeSet::new();
        m.insert(0x7D);
        m.insert(0x5D);
        m.insert(0x2C);
        m
    };
}

//pub const AFTER_NULL_CHARS: [u8; 3] = [
//    0x7D, // }
//    0x5D, // ]
//    0x2C, // ,
//    ];

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

pub const DATETIME_ID: &str = "@dt"; // -
pub const TIMESTAMP_ID: &str = "@ts"; // -

pub const MINUS_CHAR: u8 = 0x2D; // -
pub const PLUS_CHAR: u8 = 0x2B; // +

lazy_static! {
    pub static ref DIGIT_CHARS: BTreeSet<u8> = {
        let mut m = BTreeSet::new();
        m.insert(0x30);
        m.insert(0x31);
        m.insert(0x32);
        m.insert(0x33);
        m.insert(0x34);
        m.insert(0x35);
        m.insert(0x36);
        m.insert(0x37);
        m.insert(0x38);
        m.insert(0x39);
        m
    };
}

//pub const DIGIT_CHARS: [u8; 10] = [
//    0x30, // 0
//    0x31, // 1
//    0x32, // 2
//    0x33, // 3
//    0x34, // 4
//    0x35, // 5
//    0x36, // 6
//    0x37, // 7
//    0x38, // 8
//    0x39, // 9
//    ];

lazy_static! {
    pub static ref NUMERIC_CHARS: BTreeSet<u8> = {
        let mut m = BTreeSet::new();
        m.insert(0x2E);
        m.insert(0x30);
        m.insert(0x31);
        m.insert(0x32);
        m.insert(0x33);
        m.insert(0x34);
        m.insert(0x35);
        m.insert(0x36);
        m.insert(0x37);
        m.insert(0x38);
        m.insert(0x39);
        m
    };
}


//pub const NUMERIC_CHARS: [u8; 11] = [
//    0x2E, // .
//    0x30, // 0
//    0x31, // 1
//    0x32, // 2
//    0x33, // 3
//    0x34, // 4
//    0x35, // 5
//    0x36, // 6
//    0x37, // 7
//    0x38, // 8
//    0x39, // 9
//    ];
