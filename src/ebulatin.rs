use phf::phf_map;
use std::str;

static E1: phf::Map<char, u8> = phf_map! { 
    ' ' => 0x20, '!' => 0x21, '"' => 0x22, '#' => 0x23, '¤' => 0x24, '%' => 0x25, '&' => 0x26, '\'' => 0x27, '(' => 0x28, ')' => 0x29, '*' => 0x2A, '+' => 0x2B,  ',' => 0x2C, '-' => 0x2D, '.' => 0x2E, '/' => 0x2F,

    '0' => 0x30, '1' => 0x31, '2' => 0x32, '3' => 0x33, '4' => 0x34, '5' => 0x35, '6' => 0x36,  '7' => 0x37, '8' => 0x38, '9' => 0x39, ':' => 0x3A, ';' => 0x3B,  '<' => 0x3C, '=' => 0x3D, '>' => 0x3E, '?' => 0x3F,
    
    '@' => 0x40, 'A' => 0x41, 'B' => 0x42, 'C' => 0x43, 'D' => 0x44, 'E' => 0x45, 'F' => 0x46,  'G' => 0x47, 'H' => 0x48, 'I' => 0x49, 'J' => 0x4A, 'K' => 0x4B,  'L' => 0x4C, 'M' => 0x4D, 'N' => 0x4E, 'O' => 0x4F,
    
    'P' => 0x50, 'Q' => 0x51, 'R' => 0x52, 'S' => 0x53, 'T' => 0x54, 'U' => 0x55, 'V' => 0x56,  'W' => 0x57, 'X' => 0x58, 'Y' => 0x59, 'Z' => 0x5A, '[' => 0x5B, '\\' => 0x5C, ']' => 0x5D, '―' => 0x5E, '_' => 0x5F,
    
    '║' => 0x60, 'a' => 0x61, 'b' => 0x62, 'c' => 0x63, 'd' => 0x64, 'e' => 0x65, 'f' => 0x66,  'g' => 0x67, 'h' => 0x68, 'i' => 0x69, 'j' => 0x6A, 'k' => 0x6B,  'l' => 0x6C, 'm' => 0x6D, 'n' => 0x6E, 'o' => 0x6F,
    
    'p' => 0x70, 'q' => 0x71, 'r' => 0x72, 's' => 0x73, 't' => 0x74, 'u' => 0x75, 'v' => 0x76,  'w' => 0x77, 'x' => 0x78, 'y' => 0x79, 'z' => 0x7A, '{' => 0x7B,  '|' => 0x7C, '}' => 0x7D, '¯' => 0x7E, /* no 0x7F */
    
    'á' => 0x80, 'à' => 0x81, 'é' => 0x82, 'è' => 0x83, 'í' => 0x84, 'ì' => 0x85, 'ó' => 0x86,  'ò' => 0x87, 'ú' => 0x88, 'ù' => 0x89, 'Ñ' => 0x8A, 'Ç' => 0x8B,  'Ş' => 0x8C, 'ß' => 0x8D, '¡' => 0x8E, 'Ĳ' => 0x8F,
    
    'â' => 0x90, 'ä' => 0x91, 'ê' => 0x92, 'ë' => 0x93, 'î' => 0x94, 'ï' => 0x95, 'ô' => 0x96,  'ö' => 0x97, 'û' => 0x98, 'ü' => 0x99, 'ñ' => 0x9A, 'ç' => 0x9B,  'ş' => 0x9C, 'ğ' => 0x9D, 'ı' => 0x9E, 'ĳ' => 0x9F,
    
    'ª' => 0xA0, 'α' => 0xA1, '©' => 0xA2, '‰' => 0xA3, 'Ğ' => 0xA4, 'ě' => 0xA5, 'ň' => 0xA6,  'ő' => 0xA7, 'π' => 0xA8, '€' => 0xA9, '£' => 0xAA, '$' => 0xAB,  '←' => 0xAC, '↑' => 0xAD, '→' => 0xAE, '↓' => 0xAF,
    
    'º' => 0xB0, '¹' => 0xB1, '²' => 0xB2, '³' => 0xB3, '±' => 0xB4, 'İ' => 0xB5, 'ń' => 0xB6,  'ű' => 0xB7, 'µ' => 0xB8, '¿' => 0xB9, '÷' => 0xBA, '°' => 0xBB,  '¼' => 0xBC, '½' => 0xBD, '¾' => 0xBE, '§' => 0xBF,
    
    'Á' => 0xC0, 'À' => 0xC1, 'É' => 0xC2, 'È' => 0xC3, 'Í' => 0xC4, 'Ì' => 0xC5, 'Ó' => 0xC6,  'Ò' => 0xC7, 'Ú' => 0xC8, 'Ù' => 0xC9, 'Ř' => 0xCA, 'Č' => 0xCB,  'Š' => 0xCC, 'Ž' => 0xCD, 'Ð' => 0xCE, 'Ŀ' => 0xCF,
    
    'Â' => 0xD0, 'Ä' => 0xD1, 'Ê' => 0xD2, 'Ë' => 0xD3, 'Î' => 0xD4, 'Ï' => 0xD5, 'Ô' => 0xD6,  'Ö' => 0xD7, 'Û' => 0xD8, 'Ü' => 0xD9, 'ř' => 0xDA, 'č' => 0xDB,  'š' => 0xDC, 'ž' => 0xDD, 'đ' => 0xDE, 'ŀ' => 0xDF,
    
    'Ã' => 0xE0, 'Å' => 0xE1, 'Æ' => 0xE2, 'Œ' => 0xE3, 'ŷ' => 0xE4, 'Ý' => 0xE5, 'Õ' => 0xE6,  'Ø' => 0xE7, 'Ꝥ' => 0xE8, 'Ŋ' => 0xE9, 'Ŕ' => 0xEA, 'Ć' => 0xEB,  'Ś' => 0xEC, 'Ź' => 0xED, 'Ŧ' => 0xEE, 'ð' => 0xEF,
    
    'ã' => 0xF0, 'å' => 0xF1, 'æ' => 0xF2, 'œ' => 0xF3, 'ŵ' => 0xF4, 'ý' => 0xF5, 'õ' => 0xF6,  'ø' => 0xF7, 'ꝧ' => 0xF8, 'ŋ' => 0xF9, 'ŕ' => 0xFA, 'ć' => 0xFB,  'ś' => 0xFC, 'ź' => 0xFD, 'ŧ' => 0xFE
};

pub fn to_e1(source: &str) -> Vec<u8> {
    source.chars().map(|c| {
        match E1.get(&c) {
            Some(x) => *x,
            None => c as u8
        }
    }).collect()
}
