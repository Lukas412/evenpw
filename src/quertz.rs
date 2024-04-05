use crate::PwGenerator;

const LEFT_HAND: &'static [char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'q', 'r', 's', 't', 'v', 'w', 'x', 'y',
];
const LEFT_HAND_SPECIAL: &'static [char] = &['1', '2', '3', '4', '5', '6'];
const LEFT_HAND_SHIFT: &'static [char] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'Q', 'R', 'S', 'T', 'V', 'W', 'X', 'Y',
];
const LEFT_HAND_SHIFT_SPECIAL: &'static [char] = &['!', '"', '§', '$', '%', '&'];

const RIGHT_HAND: &'static [char] = &[
    'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'u', 'z', 'ö', 'ä', 'ü', ',', '.', '-',
];
const RIGHT_HAND_SPECIAL: &'static [char] = &['7', '8', '9', '0', 'ß', '+', '#'];
const RIGHT_HAND_SHIFT: &'static [char] = &[
    'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'U', 'Z', 'Ö', 'Ä', 'Ü', ';', ':', '_',
];
const RIGHT_HAND_SHIFT_SPECIAL: &'static [char] = &['/', '(', ')', '=', '?', '*', '\''];

pub struct GermanQuertzLayout;

impl PwGenerator for GermanQuertzLayout {
    fn keys(hand: bool, shift: bool, special: bool) -> &'static [char] {
        const LEFT: bool = false;
        const RIGHT: bool = true;

        match (hand, shift, special) {
            (LEFT, false, false) => LEFT_HAND,
            (LEFT, false, true) => LEFT_HAND_SPECIAL,
            (LEFT, true, false) => LEFT_HAND_SHIFT,
            (LEFT, true, true) => LEFT_HAND_SHIFT_SPECIAL,
            (RIGHT, false, false) => RIGHT_HAND,
            (RIGHT, false, true) => RIGHT_HAND_SPECIAL,
            (RIGHT, true, false) => RIGHT_HAND_SHIFT,
            (RIGHT, true, true) => RIGHT_HAND_SHIFT_SPECIAL,
        }
    }
}
