// Authentic Vector Font ported from vectorfont.bmx
use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct LineSeg {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}

pub static FONT_CHARS: [&[LineSeg]; 98] = [
    // Char 32: ' '
    &[],
    // Char 33: '!'
    &[
        LineSeg { x1: 2.0, y1: 0.0, x2: 2.0, y2: 4.0 },
        LineSeg { x1: 2.0, y1: 5.0, x2: 2.0, y2: 6.0 },
    ],
    // Char 34: '"'
    &[
        LineSeg { x1: 1.0, y1: 1.0, x2: 1.0, y2: 3.0 },
        LineSeg { x1: 3.0, y1: 1.0, x2: 3.0, y2: 3.0 },
    ],
    // Char 35: '#'
    &[
        LineSeg { x1: 1.0, y1: 0.0, x2: 1.0, y2: 6.0 },
        LineSeg { x1: 3.0, y1: 0.0, x2: 3.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 2.0, x2: 4.0, y2: 2.0 },
        LineSeg { x1: 0.0, y1: 4.0, x2: 4.0, y2: 4.0 },
    ],
    // Char 36: '$'
    &[
        LineSeg { x1: 0.0, y1: 1.0, x2: 0.0, y2: 3.0 },
        LineSeg { x1: 0.0, y1: 3.0, x2: 4.0, y2: 3.0 },
        LineSeg { x1: 4.0, y1: 3.0, x2: 4.0, y2: 5.0 },
        LineSeg { x1: 4.0, y1: 5.0, x2: 0.0, y2: 5.0 },
        LineSeg { x1: 0.0, y1: 1.0, x2: 4.0, y2: 1.0 },
        LineSeg { x1: 2.0, y1: 0.0, x2: 2.0, y2: 6.0 },
    ],
    // Char 37: '%'
    &[
        LineSeg { x1: 3.0, y1: 0.0, x2: 1.0, y2: 6.0 },
        LineSeg { x1: 1.0, y1: 1.0, x2: 1.0, y2: 2.0 },
        LineSeg { x1: 3.0, y1: 4.0, x2: 3.0, y2: 5.0 },
    ],
    // Char 38: '&'
    &[
        LineSeg { x1: 0.0, y1: 1.0, x2: 4.0, y2: 5.0 },
        LineSeg { x1: 0.0, y1: 1.0, x2: 1.0, y2: 0.0 },
        LineSeg { x1: 1.0, y1: 0.0, x2: 2.0, y2: 1.0 },
        LineSeg { x1: 2.0, y1: 1.0, x2: 0.0, y2: 4.0 },
        LineSeg { x1: 0.0, y1: 4.0, x2: 2.0, y2: 6.0 },
        LineSeg { x1: 2.0, y1: 6.0, x2: 4.0, y2: 4.0 },
    ],
    // Char 39: "'"
    &[
        LineSeg { x1: 3.0, y1: 1.0, x2: 2.0, y2: 2.0 },
    ],
    // Char 40: '('
    &[
        LineSeg { x1: 3.0, y1: 0.0, x2: 1.0, y2: 2.0 },
        LineSeg { x1: 1.0, y1: 2.0, x2: 1.0, y2: 4.0 },
        LineSeg { x1: 1.0, y1: 4.0, x2: 3.0, y2: 6.0 },
    ],
    // Char 41: ')'
    &[
        LineSeg { x1: 1.0, y1: 0.0, x2: 3.0, y2: 2.0 },
        LineSeg { x1: 3.0, y1: 2.0, x2: 3.0, y2: 4.0 },
        LineSeg { x1: 3.0, y1: 4.0, x2: 1.0, y2: 6.0 },
    ],
    // Char 42: '*'
    &[
        LineSeg { x1: 1.0, y1: 1.0, x2: 3.0, y2: 5.0 },
        LineSeg { x1: 1.0, y1: 5.0, x2: 3.0, y2: 1.0 },
        LineSeg { x1: 0.0, y1: 3.0, x2: 4.0, y2: 3.0 },
    ],
    // Char 43: '+'
    &[
        LineSeg { x1: 2.0, y1: 2.0, x2: 2.0, y2: 4.0 },
        LineSeg { x1: 1.0, y1: 3.0, x2: 3.0, y2: 3.0 },
    ],
    // Char 44: ','
    &[
        LineSeg { x1: 2.0, y1: 5.0, x2: 2.0, y2: 6.0 },
    ],
    // Char 45: '-'
    &[
        LineSeg { x1: 1.0, y1: 3.0, x2: 3.0, y2: 3.0 },
    ],
    // Char 46: '.'
    &[
        LineSeg { x1: 2.0, y1: 5.0, x2: 2.0, y2: 5.0 },
    ],
    // Char 47: '/'
    &[
        LineSeg { x1: 4.0, y1: 0.0, x2: 0.0, y2: 6.0 },
    ],
    // Char 48: '0'
    &[
        LineSeg { x1: 0.0, y1: 0.0, x2: 0.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 6.0, x2: 4.0, y2: 6.0 },
        LineSeg { x1: 4.0, y1: 6.0, x2: 4.0, y2: 0.0 },
        LineSeg { x1: 4.0, y1: 0.0, x2: 0.0, y2: 0.0 },
    ],
    // Char 49: '1'
    &[
        LineSeg { x1: 2.0, y1: 0.0, x2: 2.0, y2: 6.0 },
    ],
    // Char 50: '2'
    &[
        LineSeg { x1: 0.0, y1: 0.0, x2: 4.0, y2: 0.0 },
        LineSeg { x1: 4.0, y1: 0.0, x2: 4.0, y2: 3.0 },
        LineSeg { x1: 4.0, y1: 3.0, x2: 0.0, y2: 3.0 },
        LineSeg { x1: 0.0, y1: 3.0, x2: 0.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 6.0, x2: 4.0, y2: 6.0 },
    ],
    // Char 51: '3'
    &[
        LineSeg { x1: 0.0, y1: 0.0, x2: 4.0, y2: 0.0 },
        LineSeg { x1: 4.0, y1: 0.0, x2: 4.0, y2: 6.0 },
        LineSeg { x1: 4.0, y1: 6.0, x2: 0.0, y2: 6.0 },
        LineSeg { x1: 2.0, y1: 3.0, x2: 4.0, y2: 3.0 },
    ],
    // Char 52: '4'
    &[
        LineSeg { x1: 0.0, y1: 0.0, x2: 0.0, y2: 3.0 },
        LineSeg { x1: 0.0, y1: 3.0, x2: 4.0, y2: 3.0 },
        LineSeg { x1: 4.0, y1: 0.0, x2: 4.0, y2: 6.0 },
    ],
    // Char 53: '5'
    &[
        LineSeg { x1: 0.0, y1: 0.0, x2: 4.0, y2: 0.0 },
        LineSeg { x1: 0.0, y1: 0.0, x2: 0.0, y2: 3.0 },
        LineSeg { x1: 0.0, y1: 3.0, x2: 4.0, y2: 3.0 },
        LineSeg { x1: 4.0, y1: 3.0, x2: 4.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 6.0, x2: 4.0, y2: 6.0 },
    ],
    // Char 54: '6'
    &[
        LineSeg { x1: 0.0, y1: 0.0, x2: 0.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 3.0, x2: 4.0, y2: 3.0 },
        LineSeg { x1: 4.0, y1: 3.0, x2: 4.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 6.0, x2: 4.0, y2: 6.0 },
    ],
    // Char 55: '7'
    &[
        LineSeg { x1: 0.0, y1: 0.0, x2: 4.0, y2: 0.0 },
        LineSeg { x1: 4.0, y1: 0.0, x2: 4.0, y2: 6.0 },
    ],
    // Char 56: '8'
    &[
        LineSeg { x1: 0.0, y1: 0.0, x2: 0.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 6.0, x2: 4.0, y2: 6.0 },
        LineSeg { x1: 4.0, y1: 6.0, x2: 4.0, y2: 0.0 },
        LineSeg { x1: 4.0, y1: 0.0, x2: 0.0, y2: 0.0 },
        LineSeg { x1: 0.0, y1: 3.0, x2: 4.0, y2: 3.0 },
    ],
    // Char 57: '9'
    &[
        LineSeg { x1: 0.0, y1: 0.0, x2: 4.0, y2: 0.0 },
        LineSeg { x1: 4.0, y1: 0.0, x2: 4.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 0.0, x2: 0.0, y2: 3.0 },
        LineSeg { x1: 0.0, y1: 3.0, x2: 4.0, y2: 3.0 },
    ],
    // Char 58: ':'
    &[
        LineSeg { x1: 2.0, y1: 1.0, x2: 2.0, y2: 1.0 },
        LineSeg { x1: 2.0, y1: 5.0, x2: 2.0, y2: 5.0 },
    ],
    // Char 59: ';'
    &[
        LineSeg { x1: 2.0, y1: 1.0, x2: 2.0, y2: 1.0 },
        LineSeg { x1: 2.0, y1: 5.0, x2: 2.0, y2: 6.0 },
    ],
    // Char 60: '<'
    &[
        LineSeg { x1: 4.0, y1: 0.0, x2: 1.0, y2: 3.0 },
        LineSeg { x1: 1.0, y1: 3.0, x2: 4.0, y2: 6.0 },
    ],
    // Char 61: '='
    &[
        LineSeg { x1: 1.0, y1: 2.0, x2: 3.0, y2: 2.0 },
        LineSeg { x1: 1.0, y1: 4.0, x2: 3.0, y2: 4.0 },
    ],
    // Char 62: '>'
    &[
        LineSeg { x1: 0.0, y1: 0.0, x2: 3.0, y2: 3.0 },
        LineSeg { x1: 3.0, y1: 3.0, x2: 0.0, y2: 6.0 },
    ],
    // Char 63: '?'
    &[
        LineSeg { x1: 1.0, y1: 1.0, x2: 1.0, y2: 0.0 },
        LineSeg { x1: 1.0, y1: 0.0, x2: 3.0, y2: 0.0 },
        LineSeg { x1: 3.0, y1: 0.0, x2: 3.0, y2: 2.0 },
        LineSeg { x1: 3.0, y1: 2.0, x2: 2.0, y2: 3.0 },
        LineSeg { x1: 2.0, y1: 3.0, x2: 2.0, y2: 4.0 },
        LineSeg { x1: 2.0, y1: 5.0, x2: 2.0, y2: 6.0 },
    ],
    // Char 64: '@'
    &[
        LineSeg { x1: 2.0, y1: 2.0, x2: 2.0, y2: 4.0 },
        LineSeg { x1: 2.0, y1: 4.0, x2: 4.0, y2: 4.0 },
        LineSeg { x1: 4.0, y1: 4.0, x2: 4.0, y2: 0.0 },
        LineSeg { x1: 4.0, y1: 0.0, x2: 0.0, y2: 0.0 },
        LineSeg { x1: 0.0, y1: 0.0, x2: 0.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 6.0, x2: 4.0, y2: 6.0 },
    ],
    // Char 65: 'A'
    &[
        LineSeg { x1: 2.0, y1: 0.0, x2: 0.0, y2: 2.0 },
        LineSeg { x1: 2.0, y1: 0.0, x2: 4.0, y2: 2.0 },
        LineSeg { x1: 0.0, y1: 2.0, x2: 0.0, y2: 6.0 },
        LineSeg { x1: 4.0, y1: 2.0, x2: 4.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 3.0, x2: 4.0, y2: 3.0 },
    ],
    // Char 66: 'B'
    &[
        LineSeg { x1: 0.0, y1: 0.0, x2: 0.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 6.0, x2: 4.0, y2: 6.0 },
        LineSeg { x1: 4.0, y1: 6.0, x2: 4.0, y2: 3.0 },
        LineSeg { x1: 4.0, y1: 3.0, x2: 0.0, y2: 3.0 },
        LineSeg { x1: 0.0, y1: 0.0, x2: 3.0, y2: 0.0 },
        LineSeg { x1: 3.0, y1: 0.0, x2: 3.0, y2: 3.0 },
    ],
    // Char 67: 'C'
    &[
        LineSeg { x1: 0.0, y1: 0.0, x2: 0.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 6.0, x2: 4.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 0.0, x2: 4.0, y2: 0.0 },
    ],
    // Char 68: 'D'
    &[
        LineSeg { x1: 0.0, y1: 0.0, x2: 0.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 6.0, x2: 2.0, y2: 6.0 },
        LineSeg { x1: 2.0, y1: 6.0, x2: 4.0, y2: 4.0 },
        LineSeg { x1: 4.0, y1: 4.0, x2: 4.0, y2: 2.0 },
        LineSeg { x1: 4.0, y1: 2.0, x2: 2.0, y2: 0.0 },
        LineSeg { x1: 2.0, y1: 0.0, x2: 0.0, y2: 0.0 },
    ],
    // Char 69: 'E'
    &[
        LineSeg { x1: 0.0, y1: 0.0, x2: 0.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 6.0, x2: 4.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 0.0, x2: 4.0, y2: 0.0 },
        LineSeg { x1: 0.0, y1: 3.0, x2: 2.0, y2: 3.0 },
    ],
    // Char 70: 'F'
    &[
        LineSeg { x1: 0.0, y1: 0.0, x2: 0.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 0.0, x2: 4.0, y2: 0.0 },
        LineSeg { x1: 0.0, y1: 3.0, x2: 2.0, y2: 3.0 },
    ],
    // Char 71: 'G'
    &[
        LineSeg { x1: 0.0, y1: 0.0, x2: 0.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 6.0, x2: 4.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 0.0, x2: 4.0, y2: 0.0 },
        LineSeg { x1: 4.0, y1: 6.0, x2: 4.0, y2: 3.0 },
        LineSeg { x1: 4.0, y1: 3.0, x2: 2.0, y2: 3.0 },
    ],
    // Char 72: 'H'
    &[
        LineSeg { x1: 0.0, y1: 0.0, x2: 0.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 3.0, x2: 4.0, y2: 3.0 },
        LineSeg { x1: 4.0, y1: 0.0, x2: 4.0, y2: 6.0 },
    ],
    // Char 73: 'I'
    &[
        LineSeg { x1: 0.0, y1: 0.0, x2: 4.0, y2: 0.0 },
        LineSeg { x1: 0.0, y1: 6.0, x2: 4.0, y2: 6.0 },
        LineSeg { x1: 2.0, y1: 0.0, x2: 2.0, y2: 6.0 },
    ],
    // Char 74: 'J'
    &[
        LineSeg { x1: 3.0, y1: 0.0, x2: 4.0, y2: 0.0 },
        LineSeg { x1: 4.0, y1: 0.0, x2: 4.0, y2: 6.0 },
        LineSeg { x1: 4.0, y1: 6.0, x2: 2.0, y2: 6.0 },
        LineSeg { x1: 2.0, y1: 6.0, x2: 1.0, y2: 4.0 },
    ],
    // Char 75: 'K'
    &[
        LineSeg { x1: 0.0, y1: 0.0, x2: 0.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 3.0, x2: 2.0, y2: 3.0 },
        LineSeg { x1: 2.0, y1: 3.0, x2: 4.0, y2: 0.0 },
        LineSeg { x1: 2.0, y1: 3.0, x2: 4.0, y2: 6.0 },
    ],
    // Char 76: 'L'
    &[
        LineSeg { x1: 0.0, y1: 0.0, x2: 0.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 6.0, x2: 4.0, y2: 6.0 },
    ],
    // Char 77: 'M'
    &[
        LineSeg { x1: 0.0, y1: 0.0, x2: 0.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 0.0, x2: 2.0, y2: 3.0 },
        LineSeg { x1: 2.0, y1: 3.0, x2: 4.0, y2: 0.0 },
        LineSeg { x1: 4.0, y1: 0.0, x2: 4.0, y2: 6.0 },
    ],
    // Char 78: 'N'
    &[
        LineSeg { x1: 0.0, y1: 0.0, x2: 0.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 0.0, x2: 4.0, y2: 6.0 },
        LineSeg { x1: 4.0, y1: 6.0, x2: 4.0, y2: 0.0 },
    ],
    // Char 79: 'O'
    &[
        LineSeg { x1: 0.0, y1: 0.0, x2: 0.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 6.0, x2: 4.0, y2: 6.0 },
        LineSeg { x1: 4.0, y1: 6.0, x2: 4.0, y2: 0.0 },
        LineSeg { x1: 4.0, y1: 0.0, x2: 0.0, y2: 0.0 },
    ],
    // Char 80: 'P'
    &[
        LineSeg { x1: 0.0, y1: 0.0, x2: 0.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 0.0, x2: 4.0, y2: 0.0 },
        LineSeg { x1: 0.0, y1: 3.0, x2: 4.0, y2: 3.0 },
        LineSeg { x1: 4.0, y1: 3.0, x2: 4.0, y2: 0.0 },
    ],
    // Char 81: 'Q'
    &[
        LineSeg { x1: 0.0, y1: 0.0, x2: 0.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 6.0, x2: 2.0, y2: 6.0 },
        LineSeg { x1: 2.0, y1: 6.0, x2: 4.0, y2: 4.0 },
        LineSeg { x1: 4.0, y1: 4.0, x2: 4.0, y2: 0.0 },
        LineSeg { x1: 4.0, y1: 0.0, x2: 0.0, y2: 0.0 },
        LineSeg { x1: 4.0, y1: 6.0, x2: 2.0, y2: 4.0 },
    ],
    // Char 82: 'R'
    &[
        LineSeg { x1: 0.0, y1: 0.0, x2: 0.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 0.0, x2: 4.0, y2: 0.0 },
        LineSeg { x1: 0.0, y1: 3.0, x2: 4.0, y2: 3.0 },
        LineSeg { x1: 4.0, y1: 0.0, x2: 4.0, y2: 3.0 },
        LineSeg { x1: 2.0, y1: 3.0, x2: 4.0, y2: 6.0 },
    ],
    // Char 83: 'S'
    &[
        LineSeg { x1: 0.0, y1: 0.0, x2: 0.0, y2: 3.0 },
        LineSeg { x1: 0.0, y1: 3.0, x2: 4.0, y2: 3.0 },
        LineSeg { x1: 4.0, y1: 3.0, x2: 4.0, y2: 6.0 },
        LineSeg { x1: 4.0, y1: 6.0, x2: 0.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 0.0, x2: 4.0, y2: 0.0 },
    ],
    // Char 84: 'T'
    &[
        LineSeg { x1: 0.0, y1: 0.0, x2: 4.0, y2: 0.0 },
        LineSeg { x1: 2.0, y1: 0.0, x2: 2.0, y2: 6.0 },
    ],
    // Char 85: 'U'
    &[
        LineSeg { x1: 0.0, y1: 0.0, x2: 0.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 6.0, x2: 4.0, y2: 6.0 },
        LineSeg { x1: 4.0, y1: 6.0, x2: 4.0, y2: 0.0 },
    ],
    // Char 86: 'V'
    &[
        LineSeg { x1: 0.0, y1: 0.0, x2: 0.0, y2: 3.0 },
        LineSeg { x1: 0.0, y1: 3.0, x2: 2.0, y2: 6.0 },
        LineSeg { x1: 2.0, y1: 6.0, x2: 4.0, y2: 3.0 },
        LineSeg { x1: 4.0, y1: 3.0, x2: 4.0, y2: 0.0 },
    ],
    // Char 87: 'W'
    &[
        LineSeg { x1: 0.0, y1: 0.0, x2: 1.0, y2: 6.0 },
        LineSeg { x1: 1.0, y1: 6.0, x2: 2.0, y2: 4.0 },
        LineSeg { x1: 2.0, y1: 4.0, x2: 3.0, y2: 6.0 },
        LineSeg { x1: 3.0, y1: 6.0, x2: 4.0, y2: 0.0 },
    ],
    // Char 88: 'X'
    &[
        LineSeg { x1: 0.0, y1: 0.0, x2: 4.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 6.0, x2: 4.0, y2: 0.0 },
    ],
    // Char 89: 'Y'
    &[
        LineSeg { x1: 0.0, y1: 0.0, x2: 2.0, y2: 3.0 },
        LineSeg { x1: 2.0, y1: 3.0, x2: 4.0, y2: 0.0 },
        LineSeg { x1: 2.0, y1: 3.0, x2: 2.0, y2: 6.0 },
    ],
    // Char 90: 'Z'
    &[
        LineSeg { x1: 0.0, y1: 0.0, x2: 4.0, y2: 0.0 },
        LineSeg { x1: 4.0, y1: 0.0, x2: 0.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 6.0, x2: 4.0, y2: 6.0 },
    ],
    // Char 91: '['
    &[
        LineSeg { x1: 3.0, y1: 0.0, x2: 1.0, y2: 0.0 },
        LineSeg { x1: 1.0, y1: 0.0, x2: 1.0, y2: 6.0 },
        LineSeg { x1: 1.0, y1: 6.0, x2: 3.0, y2: 6.0 },
    ],
    // Char 92: '\\'
    &[
        LineSeg { x1: 0.0, y1: 0.0, x2: 4.0, y2: 6.0 },
    ],
    // Char 93: ']'
    &[
        LineSeg { x1: 1.0, y1: 0.0, x2: 3.0, y2: 0.0 },
        LineSeg { x1: 3.0, y1: 0.0, x2: 3.0, y2: 6.0 },
        LineSeg { x1: 3.0, y1: 6.0, x2: 1.0, y2: 6.0 },
    ],
    // Char 94: '^'
    &[
        LineSeg { x1: 1.0, y1: 2.0, x2: 2.0, y2: 0.0 },
        LineSeg { x1: 2.0, y1: 0.0, x2: 3.0, y2: 2.0 },
    ],
    // Char 95: '_'
    &[
        LineSeg { x1: 0.0, y1: 7.0, x2: 4.0, y2: 7.0 },
    ],
    // Char 96: '`'
    &[
        LineSeg { x1: 1.0, y1: 1.0, x2: 2.0, y2: 2.0 },
    ],
    // Char 97: 'a'
    &[
        LineSeg { x1: 0.0, y1: 2.0, x2: 4.0, y2: 2.0 },
        LineSeg { x1: 4.0, y1: 2.0, x2: 4.0, y2: 6.0 },
        LineSeg { x1: 4.0, y1: 6.0, x2: 0.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 6.0, x2: 0.0, y2: 3.0 },
        LineSeg { x1: 0.0, y1: 3.0, x2: 4.0, y2: 3.0 },
    ],
    // Char 98: 'b'
    &[
        LineSeg { x1: 0.0, y1: 0.0, x2: 0.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 6.0, x2: 4.0, y2: 6.0 },
        LineSeg { x1: 4.0, y1: 6.0, x2: 4.0, y2: 2.0 },
        LineSeg { x1: 4.0, y1: 2.0, x2: 0.0, y2: 2.0 },
    ],
    // Char 99: 'c'
    &[
        LineSeg { x1: 0.0, y1: 2.0, x2: 0.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 6.0, x2: 4.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 2.0, x2: 4.0, y2: 2.0 },
    ],
    // Char 100: 'd'
    &[
        LineSeg { x1: 4.0, y1: 0.0, x2: 4.0, y2: 6.0 },
        LineSeg { x1: 4.0, y1: 6.0, x2: 0.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 6.0, x2: 0.0, y2: 2.0 },
        LineSeg { x1: 0.0, y1: 2.0, x2: 4.0, y2: 2.0 },
    ],
    // Char 101: 'e'
    &[
        LineSeg { x1: 4.0, y1: 6.0, x2: 0.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 6.0, x2: 0.0, y2: 2.0 },
        LineSeg { x1: 0.0, y1: 2.0, x2: 4.0, y2: 2.0 },
        LineSeg { x1: 4.0, y1: 2.0, x2: 4.0, y2: 3.0 },
        LineSeg { x1: 4.0, y1: 3.0, x2: 1.0, y2: 3.0 },
    ],
    // Char 102: 'f'
    &[
        LineSeg { x1: 4.0, y1: 0.0, x2: 2.0, y2: 0.0 },
        LineSeg { x1: 2.0, y1: 0.0, x2: 2.0, y2: 6.0 },
        LineSeg { x1: 1.0, y1: 2.0, x2: 3.0, y2: 2.0 },
    ],
    // Char 103: 'g'
    &[
        LineSeg { x1: 0.0, y1: 7.0, x2: 4.0, y2: 7.0 },
        LineSeg { x1: 4.0, y1: 7.0, x2: 4.0, y2: 2.0 },
        LineSeg { x1: 4.0, y1: 2.0, x2: 0.0, y2: 2.0 },
        LineSeg { x1: 0.0, y1: 2.0, x2: 0.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 6.0, x2: 4.0, y2: 6.0 },
    ],
    // Char 104: 'h'
    &[
        LineSeg { x1: 0.0, y1: 0.0, x2: 0.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 2.0, x2: 4.0, y2: 2.0 },
        LineSeg { x1: 4.0, y1: 2.0, x2: 4.0, y2: 6.0 },
    ],
    // Char 105: 'i'
    &[
        LineSeg { x1: 2.0, y1: 2.0, x2: 2.0, y2: 6.0 },
        LineSeg { x1: 2.0, y1: 1.0, x2: 2.0, y2: 1.0 },
    ],
    // Char 106: 'j'
    &[
        LineSeg { x1: 3.0, y1: 1.0, x2: 3.0, y2: 1.0 },
        LineSeg { x1: 3.0, y1: 2.0, x2: 3.0, y2: 7.0 },
        LineSeg { x1: 3.0, y1: 7.0, x2: 0.0, y2: 7.0 },
        LineSeg { x1: 0.0, y1: 7.0, x2: 0.0, y2: 5.0 },
    ],
    // Char 107: 'k'
    &[
        LineSeg { x1: 0.0, y1: 0.0, x2: 0.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 4.0, x2: 3.0, y2: 2.0 },
        LineSeg { x1: 0.0, y1: 4.0, x2: 3.0, y2: 6.0 },
    ],
    // Char 108: 'l'
    &[
        LineSeg { x1: 2.0, y1: 0.0, x2: 2.0, y2: 6.0 },
    ],
    // Char 109: 'm'
    &[
        LineSeg { x1: 0.0, y1: 6.0, x2: 0.0, y2: 2.0 },
        LineSeg { x1: 0.0, y1: 2.0, x2: 4.0, y2: 2.0 },
        LineSeg { x1: 2.0, y1: 2.0, x2: 2.0, y2: 4.0 },
        LineSeg { x1: 4.0, y1: 2.0, x2: 4.0, y2: 6.0 },
    ],
    // Char 110: 'n'
    &[
        LineSeg { x1: 0.0, y1: 2.0, x2: 0.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 3.0, x2: 1.0, y2: 2.0 },
        LineSeg { x1: 1.0, y1: 2.0, x2: 4.0, y2: 2.0 },
        LineSeg { x1: 4.0, y1: 2.0, x2: 4.0, y2: 6.0 },
    ],
    // Char 111: 'o'
    &[
        LineSeg { x1: 0.0, y1: 2.0, x2: 0.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 6.0, x2: 4.0, y2: 6.0 },
        LineSeg { x1: 4.0, y1: 6.0, x2: 4.0, y2: 2.0 },
        LineSeg { x1: 4.0, y1: 2.0, x2: 0.0, y2: 2.0 },
    ],
    // Char 112: 'p'
    &[
        LineSeg { x1: 0.0, y1: 2.0, x2: 0.0, y2: 7.0 },
        LineSeg { x1: 0.0, y1: 2.0, x2: 4.0, y2: 2.0 },
        LineSeg { x1: 0.0, y1: 6.0, x2: 4.0, y2: 6.0 },
        LineSeg { x1: 4.0, y1: 2.0, x2: 4.0, y2: 6.0 },
    ],
    // Char 113: 'q'
    &[
        LineSeg { x1: 0.0, y1: 2.0, x2: 0.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 6.0, x2: 4.0, y2: 6.0 },
        LineSeg { x1: 4.0, y1: 2.0, x2: 4.0, y2: 7.0 },
        LineSeg { x1: 0.0, y1: 2.0, x2: 4.0, y2: 2.0 },
    ],
    // Char 114: 'r'
    &[
        LineSeg { x1: 0.0, y1: 2.0, x2: 0.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 3.0, x2: 1.0, y2: 2.0 },
        LineSeg { x1: 1.0, y1: 2.0, x2: 4.0, y2: 2.0 },
    ],
    // Char 115: 's'
    &[
        LineSeg { x1: 4.0, y1: 2.0, x2: 0.0, y2: 2.0 },
        LineSeg { x1: 0.0, y1: 2.0, x2: 0.0, y2: 4.0 },
        LineSeg { x1: 0.0, y1: 4.0, x2: 4.0, y2: 4.0 },
        LineSeg { x1: 4.0, y1: 4.0, x2: 4.0, y2: 6.0 },
        LineSeg { x1: 4.0, y1: 6.0, x2: 0.0, y2: 6.0 },
    ],
    // Char 116: 't'
    &[
        LineSeg { x1: 0.0, y1: 2.0, x2: 4.0, y2: 2.0 },
        LineSeg { x1: 2.0, y1: 0.0, x2: 2.0, y2: 6.0 },
    ],
    // Char 117: 'u'
    &[
        LineSeg { x1: 0.0, y1: 2.0, x2: 0.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 6.0, x2: 4.0, y2: 6.0 },
        LineSeg { x1: 4.0, y1: 6.0, x2: 4.0, y2: 2.0 },
    ],
    // Char 118: 'v'
    &[
        LineSeg { x1: 0.0, y1: 2.0, x2: 2.0, y2: 6.0 },
        LineSeg { x1: 2.0, y1: 6.0, x2: 4.0, y2: 2.0 },
    ],
    // Char 119: 'w'
    &[
        LineSeg { x1: 0.0, y1: 2.0, x2: 1.0, y2: 6.0 },
        LineSeg { x1: 1.0, y1: 6.0, x2: 2.0, y2: 4.0 },
        LineSeg { x1: 2.0, y1: 4.0, x2: 3.0, y2: 6.0 },
        LineSeg { x1: 3.0, y1: 6.0, x2: 4.0, y2: 2.0 },
    ],
    // Char 120: 'x'
    &[
        LineSeg { x1: 0.0, y1: 2.0, x2: 4.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 6.0, x2: 4.0, y2: 2.0 },
    ],
    // Char 121: 'y'
    &[
        LineSeg { x1: 0.0, y1: 2.0, x2: 2.0, y2: 5.0 },
        LineSeg { x1: 4.0, y1: 2.0, x2: 1.0, y2: 7.0 },
    ],
    // Char 122: 'z'
    &[
        LineSeg { x1: 0.0, y1: 2.0, x2: 4.0, y2: 2.0 },
        LineSeg { x1: 4.0, y1: 2.0, x2: 0.0, y2: 6.0 },
        LineSeg { x1: 0.0, y1: 6.0, x2: 4.0, y2: 6.0 },
    ],
    // Char 123: '{'
    &[
        LineSeg { x1: 3.0, y1: 0.0, x2: 2.0, y2: 0.0 },
        LineSeg { x1: 2.0, y1: 0.0, x2: 2.0, y2: 6.0 },
        LineSeg { x1: 2.0, y1: 6.0, x2: 3.0, y2: 6.0 },
        LineSeg { x1: 1.0, y1: 3.0, x2: 2.0, y2: 3.0 },
    ],
    // Char 124: '|'
    &[
        LineSeg { x1: 2.0, y1: 0.0, x2: 2.0, y2: 6.0 },
    ],
    // Char 125: '}'
    &[
        LineSeg { x1: 1.0, y1: 0.0, x2: 2.0, y2: 0.0 },
        LineSeg { x1: 2.0, y1: 0.0, x2: 2.0, y2: 6.0 },
        LineSeg { x1: 2.0, y1: 6.0, x2: 1.0, y2: 6.0 },
        LineSeg { x1: 2.0, y1: 3.0, x2: 3.0, y2: 3.0 },
    ],
    // Char 126: '~'
    &[
        LineSeg { x1: 0.0, y1: 3.0, x2: 0.0, y2: 1.0 },
        LineSeg { x1: 0.0, y1: 1.0, x2: 2.0, y2: 1.0 },
        LineSeg { x1: 2.0, y1: 1.0, x2: 2.0, y2: 3.0 },
        LineSeg { x1: 2.0, y1: 3.0, x2: 4.0, y2: 3.0 },
        LineSeg { x1: 4.0, y1: 3.0, x2: 4.0, y2: 1.0 },
    ],
    // Char 127: 127
    &[
        LineSeg { x1: 3.0, y1: 1.0, x2: 0.0, y2: 3.0 },
        LineSeg { x1: 0.0, y1: 3.0, x2: 3.0, y2: 5.0 },
        LineSeg { x1: 0.0, y1: 3.0, x2: 4.0, y2: 3.0 },
    ],
    // Char 128: 128
    &[
        LineSeg { x1: 0.0, y1: 4.0, x2: 2.0, y2: 6.0 },
        LineSeg { x1: 2.0, y1: 6.0, x2: 4.0, y2: 0.0 },
    ],
    // Char 129: 129
    &[
        LineSeg { x1: 1.0, y1: 1.0, x2: 4.0, y2: 3.0 },
        LineSeg { x1: 4.0, y1: 3.0, x2: 1.0, y2: 5.0 },
        LineSeg { x1: 0.0, y1: 3.0, x2: 4.0, y2: 3.0 },
    ],

];

pub fn draw_char(c: char, x: f32, y: f32, scale: f32, color: Color, line_width: f32) {
    let idx = (c as usize).wrapping_sub(32);
    if idx < FONT_CHARS.len() {
        for seg in FONT_CHARS[idx] {
            draw_line(
                x + seg.x1 * scale,
                y + seg.y1 * scale,
                x + seg.x2 * scale,
                y + seg.y2 * scale,
                line_width,
                color,
            );
        }
    }
}

pub fn string_width(s: &str, scale: f32) -> f32 {
    if s.is_empty() {
        0.0
    } else {
        (s.len() as f32 * 5.0 - 1.0) * scale
    }
}

pub fn draw_string(s: &str, x: f32, y: f32, scale: f32, color: Color, line_width: f32) {
    for (i, c) in s.chars().enumerate() {
        draw_char(c, x + (i as f32) * 5.0 * scale, y, scale, color, line_width);
    }
}

pub fn draw_string_centered(s: &str, cx: f32, y: f32, scale: f32, color: Color, line_width: f32) {
    let width = string_width(s, scale);
    draw_string(s, cx - width * 0.5, y, scale, color, line_width);
}
