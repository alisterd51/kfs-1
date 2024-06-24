use crate::driver::keyboard::ScanCodeValue;

use super::{Keymap, KeymapSet, KeymapValue};

// https://github.com/Stichting-MINIX-Research-Foundation/minix/blob/4db99f4012570a577414fe2a43697b2f239b699e/minix/drivers/tty/tty/keymaps/us-std.src
pub const US_QUERTY_KEYMAP: Keymap = {
    let mut keymap = [None; 0x80];
    keymap[ScanCodeValue::A as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Lowercase('a'),
        shift: KeymapValue::Ascii('A'),
        alt1: KeymapValue::Alt('a'),
        alt2: KeymapValue::Alt('a'),
        alt_shift: KeymapValue::Alt('A'),
        ctrl: KeymapValue::Control('A'),
    });
    keymap[ScanCodeValue::B as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Lowercase('b'),
        shift: KeymapValue::Ascii('B'),
        alt1: KeymapValue::Alt('b'),
        alt2: KeymapValue::Alt('b'),
        alt_shift: KeymapValue::Alt('B'),
        ctrl: KeymapValue::Control('B'),
    });
    keymap[ScanCodeValue::C as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Lowercase('c'),
        shift: KeymapValue::Ascii('C'),
        alt1: KeymapValue::Alt('c'),
        alt2: KeymapValue::Alt('c'),
        alt_shift: KeymapValue::Alt('C'),
        ctrl: KeymapValue::Control('C'),
    });
    keymap[ScanCodeValue::D as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Lowercase('d'),
        shift: KeymapValue::Ascii('D'),
        alt1: KeymapValue::Alt('d'),
        alt2: KeymapValue::Alt('d'),
        alt_shift: KeymapValue::Alt('D'),
        ctrl: KeymapValue::Control('D'),
    });
    keymap[ScanCodeValue::E as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Lowercase('e'),
        shift: KeymapValue::Ascii('E'),
        alt1: KeymapValue::Alt('e'),
        alt2: KeymapValue::Alt('e'),
        alt_shift: KeymapValue::Alt('E'),
        ctrl: KeymapValue::Control('E'),
    });
    keymap[ScanCodeValue::F as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Lowercase('f'),
        shift: KeymapValue::Ascii('F'),
        alt1: KeymapValue::Alt('f'),
        alt2: KeymapValue::Alt('f'),
        alt_shift: KeymapValue::Alt('F'),
        ctrl: KeymapValue::Control('F'),
    });
    keymap[ScanCodeValue::G as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Lowercase('g'),
        shift: KeymapValue::Ascii('G'),
        alt1: KeymapValue::Alt('g'),
        alt2: KeymapValue::Alt('g'),
        alt_shift: KeymapValue::Alt('G'),
        ctrl: KeymapValue::Control('G'),
    });
    keymap[ScanCodeValue::H as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Lowercase('h'),
        shift: KeymapValue::Ascii('H'),
        alt1: KeymapValue::Alt('h'),
        alt2: KeymapValue::Alt('h'),
        alt_shift: KeymapValue::Alt('H'),
        ctrl: KeymapValue::Control('H'),
    });
    keymap[ScanCodeValue::I as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Lowercase('i'),
        shift: KeymapValue::Ascii('I'),
        alt1: KeymapValue::Alt('i'),
        alt2: KeymapValue::Alt('i'),
        alt_shift: KeymapValue::Alt('I'),
        ctrl: KeymapValue::Control('I'),
    });
    keymap[ScanCodeValue::J as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Lowercase('j'),
        shift: KeymapValue::Ascii('J'),
        alt1: KeymapValue::Alt('j'),
        alt2: KeymapValue::Alt('j'),
        alt_shift: KeymapValue::Alt('J'),
        ctrl: KeymapValue::Control('J'),
    });
    keymap[ScanCodeValue::K as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Lowercase('k'),
        shift: KeymapValue::Ascii('K'),
        alt1: KeymapValue::Alt('k'),
        alt2: KeymapValue::Alt('k'),
        alt_shift: KeymapValue::Alt('K'),
        ctrl: KeymapValue::Control('K'),
    });
    keymap[ScanCodeValue::L as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Lowercase('l'),
        shift: KeymapValue::Ascii('L'),
        alt1: KeymapValue::Alt('l'),
        alt2: KeymapValue::Alt('l'),
        alt_shift: KeymapValue::Alt('L'),
        ctrl: KeymapValue::Control('L'),
    });
    keymap[ScanCodeValue::M as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Lowercase('m'),
        shift: KeymapValue::Ascii('M'),
        alt1: KeymapValue::Alt('m'),
        alt2: KeymapValue::Alt('m'),
        alt_shift: KeymapValue::Alt('M'),
        ctrl: KeymapValue::Control('M'),
    });
    keymap[ScanCodeValue::N as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Lowercase('n'),
        shift: KeymapValue::Ascii('N'),
        alt1: KeymapValue::Alt('n'),
        alt2: KeymapValue::Alt('n'),
        alt_shift: KeymapValue::Alt('N'),
        ctrl: KeymapValue::Control('N'),
    });
    keymap[ScanCodeValue::O as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Lowercase('o'),
        shift: KeymapValue::Ascii('O'),
        alt1: KeymapValue::Alt('o'),
        alt2: KeymapValue::Alt('o'),
        alt_shift: KeymapValue::Alt('O'),
        ctrl: KeymapValue::Control('O'),
    });
    keymap[ScanCodeValue::P as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Lowercase('p'),
        shift: KeymapValue::Ascii('P'),
        alt1: KeymapValue::Alt('p'),
        alt2: KeymapValue::Alt('p'),
        alt_shift: KeymapValue::Alt('P'),
        ctrl: KeymapValue::Control('P'),
    });
    keymap[ScanCodeValue::Q as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Lowercase('q'),
        shift: KeymapValue::Ascii('Q'),
        alt1: KeymapValue::Alt('q'),
        alt2: KeymapValue::Alt('q'),
        alt_shift: KeymapValue::Alt('Q'),
        ctrl: KeymapValue::Control('Q'),
    });
    keymap[ScanCodeValue::R as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Lowercase('r'),
        shift: KeymapValue::Ascii('R'),
        alt1: KeymapValue::Alt('r'),
        alt2: KeymapValue::Alt('r'),
        alt_shift: KeymapValue::Alt('R'),
        ctrl: KeymapValue::Control('R'),
    });
    keymap[ScanCodeValue::S as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Lowercase('s'),
        shift: KeymapValue::Ascii('S'),
        alt1: KeymapValue::Alt('s'),
        alt2: KeymapValue::Alt('s'),
        alt_shift: KeymapValue::Alt('S'),
        ctrl: KeymapValue::Control('S'),
    });
    keymap[ScanCodeValue::T as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Lowercase('t'),
        shift: KeymapValue::Ascii('T'),
        alt1: KeymapValue::Alt('t'),
        alt2: KeymapValue::Alt('t'),
        alt_shift: KeymapValue::Alt('T'),
        ctrl: KeymapValue::Control('T'),
    });
    keymap[ScanCodeValue::U as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Lowercase('u'),
        shift: KeymapValue::Ascii('U'),
        alt1: KeymapValue::Alt('u'),
        alt2: KeymapValue::Alt('u'),
        alt_shift: KeymapValue::Alt('U'),
        ctrl: KeymapValue::Control('U'),
    });
    keymap[ScanCodeValue::V as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Lowercase('v'),
        shift: KeymapValue::Ascii('V'),
        alt1: KeymapValue::Alt('v'),
        alt2: KeymapValue::Alt('v'),
        alt_shift: KeymapValue::Alt('V'),
        ctrl: KeymapValue::Control('V'),
    });
    keymap[ScanCodeValue::W as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Lowercase('w'),
        shift: KeymapValue::Ascii('W'),
        alt1: KeymapValue::Alt('w'),
        alt2: KeymapValue::Alt('w'),
        alt_shift: KeymapValue::Alt('W'),
        ctrl: KeymapValue::Control('W'),
    });
    keymap[ScanCodeValue::X as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Lowercase('x'),
        shift: KeymapValue::Ascii('X'),
        alt1: KeymapValue::Alt('x'),
        alt2: KeymapValue::Alt('x'),
        alt_shift: KeymapValue::Alt('X'),
        ctrl: KeymapValue::Control('X'),
    });
    keymap[ScanCodeValue::Y as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Lowercase('y'),
        shift: KeymapValue::Ascii('Y'),
        alt1: KeymapValue::Alt('y'),
        alt2: KeymapValue::Alt('y'),
        alt_shift: KeymapValue::Alt('Y'),
        ctrl: KeymapValue::Control('Y'),
    });
    keymap[ScanCodeValue::Z as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Lowercase('z'),
        shift: KeymapValue::Ascii('Z'),
        alt1: KeymapValue::Alt('z'),
        alt2: KeymapValue::Alt('z'),
        alt_shift: KeymapValue::Alt('Z'),
        ctrl: KeymapValue::Control('Z'),
    });
    keymap[ScanCodeValue::One as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Ascii('1'),
        shift: KeymapValue::Ascii('!'),
        alt1: KeymapValue::Alt('1'),
        alt2: KeymapValue::Alt('1'),
        alt_shift: KeymapValue::Alt('!'),
        ctrl: KeymapValue::Control('A'),
    });
    keymap[ScanCodeValue::Two as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Ascii('2'),
        shift: KeymapValue::Ascii('@'),
        alt1: KeymapValue::Alt('2'),
        alt2: KeymapValue::Alt('2'),
        alt_shift: KeymapValue::Alt('@'),
        ctrl: KeymapValue::Control('@'),
    });
    keymap[ScanCodeValue::Three as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Ascii('3'),
        shift: KeymapValue::Ascii('#'),
        alt1: KeymapValue::Alt('3'),
        alt2: KeymapValue::Alt('3'),
        alt_shift: KeymapValue::Alt('#'),
        ctrl: KeymapValue::Control('C'),
    });
    keymap[ScanCodeValue::Four as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Ascii('4'),
        shift: KeymapValue::Ascii('$'),
        alt1: KeymapValue::Alt('4'),
        alt2: KeymapValue::Alt('4'),
        alt_shift: KeymapValue::Alt('$'),
        ctrl: KeymapValue::Control('D'),
    });
    keymap[ScanCodeValue::Five as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Ascii('5'),
        shift: KeymapValue::Ascii('%'),
        alt1: KeymapValue::Alt('5'),
        alt2: KeymapValue::Alt('5'),
        alt_shift: KeymapValue::Alt('%'),
        ctrl: KeymapValue::Control('E'),
    });
    keymap[ScanCodeValue::Six as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Ascii('6'),
        shift: KeymapValue::Ascii('^'),
        alt1: KeymapValue::Alt('6'),
        alt2: KeymapValue::Alt('6'),
        alt_shift: KeymapValue::Alt('^'),
        ctrl: KeymapValue::Control('^'),
    });
    keymap[ScanCodeValue::Seven as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Ascii('7'),
        shift: KeymapValue::Ascii('&'),
        alt1: KeymapValue::Alt('7'),
        alt2: KeymapValue::Alt('7'),
        alt_shift: KeymapValue::Alt('&'),
        ctrl: KeymapValue::Control('G'),
    });
    keymap[ScanCodeValue::Eight as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Ascii('8'),
        shift: KeymapValue::Ascii('*'),
        alt1: KeymapValue::Alt('8'),
        alt2: KeymapValue::Alt('8'),
        alt_shift: KeymapValue::Alt('*'),
        ctrl: KeymapValue::Control('H'),
    });
    keymap[ScanCodeValue::Nine as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Ascii('9'),
        shift: KeymapValue::Ascii('('),
        alt1: KeymapValue::Alt('9'),
        alt2: KeymapValue::Alt('9'),
        alt_shift: KeymapValue::Alt('('),
        ctrl: KeymapValue::Control('I'),
    });
    keymap[ScanCodeValue::Zero as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Ascii('0'),
        shift: KeymapValue::Ascii(')'),
        alt1: KeymapValue::Alt('0'),
        alt2: KeymapValue::Alt('0'),
        alt_shift: KeymapValue::Alt(')'),
        ctrl: KeymapValue::Control('@'),
    });
    keymap[ScanCodeValue::Enter as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Control('M'),
        shift: KeymapValue::Control('M'),
        alt1: KeymapValue::ControlAlt('M'),
        alt2: KeymapValue::ControlAlt('M'),
        alt_shift: KeymapValue::ControlAlt('M'),
        ctrl: KeymapValue::Control('J'),
    });
    keymap[ScanCodeValue::Escape as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Control('['),
        shift: KeymapValue::Control('['),
        alt1: KeymapValue::ControlAlt('['),
        alt2: KeymapValue::ControlAlt('['),
        alt_shift: KeymapValue::ControlAlt('['),
        ctrl: KeymapValue::Control('['),
    });
    keymap[ScanCodeValue::Backspace as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Control('H'),
        shift: KeymapValue::Control('H'),
        alt1: KeymapValue::ControlAlt('H'),
        alt2: KeymapValue::ControlAlt('H'),
        alt_shift: KeymapValue::ControlAlt('H'),
        ctrl: KeymapValue::Delete,
    });
    keymap[ScanCodeValue::Tab as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Control('I'),
        shift: KeymapValue::Control('I'),
        alt1: KeymapValue::ControlAlt('I'),
        alt2: KeymapValue::ControlAlt('I'),
        alt_shift: KeymapValue::ControlAlt('I'),
        ctrl: KeymapValue::Control('I'),
    });
    keymap[ScanCodeValue::Space as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Ascii(' '),
        shift: KeymapValue::Ascii(' '),
        alt1: KeymapValue::Alt(' '),
        alt2: KeymapValue::Alt(' '),
        alt_shift: KeymapValue::Alt(' '),
        ctrl: KeymapValue::Control('@'),
    });
    keymap[ScanCodeValue::Min as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Ascii('-'),
        shift: KeymapValue::Ascii('_'),
        alt1: KeymapValue::Alt('-'),
        alt2: KeymapValue::Alt('-'),
        alt_shift: KeymapValue::Alt('_'),
        ctrl: KeymapValue::Control('_'),
    });
    keymap[ScanCodeValue::Equal as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Ascii('='),
        shift: KeymapValue::Ascii('+'),
        alt1: KeymapValue::Alt('='),
        alt2: KeymapValue::Alt('='),
        alt_shift: KeymapValue::Alt('+'),
        ctrl: KeymapValue::Control('@'),
    });
    keymap[ScanCodeValue::LeftSquareBracket as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Ascii('['),
        shift: KeymapValue::Ascii('{'),
        alt1: KeymapValue::Alt('['),
        alt2: KeymapValue::Alt('['),
        alt_shift: KeymapValue::Alt('{'),
        ctrl: KeymapValue::Control('['),
    });
    keymap[ScanCodeValue::RightSquareBracket as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Ascii(']'),
        shift: KeymapValue::Ascii('}'),
        alt1: KeymapValue::Alt(']'),
        alt2: KeymapValue::Alt(']'),
        alt_shift: KeymapValue::Alt('}'),
        ctrl: KeymapValue::Control(']'),
    });
    keymap[ScanCodeValue::BackSlash as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Ascii('\\'),
        shift: KeymapValue::Ascii('|'),
        alt1: KeymapValue::Alt('\\'),
        alt2: KeymapValue::Alt('\\'),
        alt_shift: KeymapValue::Alt('|'),
        ctrl: KeymapValue::Control('\\'),
    });
    keymap[ScanCodeValue::Semicolon as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Ascii(';'),
        shift: KeymapValue::Ascii(':'),
        alt1: KeymapValue::Alt(';'),
        alt2: KeymapValue::Alt(';'),
        alt_shift: KeymapValue::Alt(':'),
        ctrl: KeymapValue::Control('@'),
    });
    keymap[ScanCodeValue::SingleQuote as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Ascii('\''),
        shift: KeymapValue::Ascii('"'),
        alt1: KeymapValue::Alt('\''),
        alt2: KeymapValue::Alt('\''),
        alt_shift: KeymapValue::Alt('"'),
        ctrl: KeymapValue::Control('@'),
    });
    keymap[ScanCodeValue::BackTick as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Ascii('`'),
        shift: KeymapValue::Ascii('~'),
        alt1: KeymapValue::Alt('`'),
        alt2: KeymapValue::Alt('`'),
        alt_shift: KeymapValue::Alt('~'),
        ctrl: KeymapValue::Control('@'),
    });
    keymap[ScanCodeValue::Comma as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Ascii(','),
        shift: KeymapValue::Ascii('<'),
        alt1: KeymapValue::Alt(','),
        alt2: KeymapValue::Alt(','),
        alt_shift: KeymapValue::Alt('<'),
        ctrl: KeymapValue::Control('@'),
    });
    keymap[ScanCodeValue::Dot as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Ascii('.'),
        shift: KeymapValue::Ascii('>'),
        alt1: KeymapValue::Alt('.'),
        alt2: KeymapValue::Alt('.'),
        alt_shift: KeymapValue::Alt('>'),
        ctrl: KeymapValue::Control('@'),
    });
    keymap[ScanCodeValue::Slash as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Ascii('/'),
        shift: KeymapValue::Ascii('?'),
        alt1: KeymapValue::Alt('/'),
        alt2: KeymapValue::Alt('/'),
        alt_shift: KeymapValue::Alt('?'),
        ctrl: KeymapValue::Control('@'),
    });
    keymap[ScanCodeValue::CapsLock as usize] = Some(KeymapSet {
        not_shift: KeymapValue::CapsLock,
        shift: KeymapValue::CapsLock,
        alt1: KeymapValue::CapsLock,
        alt2: KeymapValue::CapsLock,
        alt_shift: KeymapValue::CapsLock,
        ctrl: KeymapValue::CapsLock,
    });
    keymap[ScanCodeValue::F1 as usize] = Some(KeymapSet {
        not_shift: KeymapValue::F1,
        shift: KeymapValue::SF1,
        alt1: KeymapValue::AF1,
        alt2: KeymapValue::AF1,
        alt_shift: KeymapValue::ASF1,
        ctrl: KeymapValue::CF1,
    });
    keymap[ScanCodeValue::F2 as usize] = Some(KeymapSet {
        not_shift: KeymapValue::F2,
        shift: KeymapValue::SF2,
        alt1: KeymapValue::AF2,
        alt2: KeymapValue::AF2,
        alt_shift: KeymapValue::ASF2,
        ctrl: KeymapValue::CF2,
    });
    keymap[ScanCodeValue::F3 as usize] = Some(KeymapSet {
        not_shift: KeymapValue::F3,
        shift: KeymapValue::SF3,
        alt1: KeymapValue::AF3,
        alt2: KeymapValue::AF3,
        alt_shift: KeymapValue::ASF3,
        ctrl: KeymapValue::CF3,
    });
    keymap[ScanCodeValue::F4 as usize] = Some(KeymapSet {
        not_shift: KeymapValue::F4,
        shift: KeymapValue::SF4,
        alt1: KeymapValue::AF4,
        alt2: KeymapValue::AF4,
        alt_shift: KeymapValue::ASF4,
        ctrl: KeymapValue::CF4,
    });
    keymap[ScanCodeValue::F5 as usize] = Some(KeymapSet {
        not_shift: KeymapValue::F5,
        shift: KeymapValue::SF5,
        alt1: KeymapValue::AF5,
        alt2: KeymapValue::AF5,
        alt_shift: KeymapValue::ASF5,
        ctrl: KeymapValue::CF5,
    });
    keymap[ScanCodeValue::F6 as usize] = Some(KeymapSet {
        not_shift: KeymapValue::F6,
        shift: KeymapValue::SF6,
        alt1: KeymapValue::AF6,
        alt2: KeymapValue::AF6,
        alt_shift: KeymapValue::ASF6,
        ctrl: KeymapValue::CF6,
    });
    keymap[ScanCodeValue::F7 as usize] = Some(KeymapSet {
        not_shift: KeymapValue::F7,
        shift: KeymapValue::SF7,
        alt1: KeymapValue::AF7,
        alt2: KeymapValue::AF7,
        alt_shift: KeymapValue::ASF7,
        ctrl: KeymapValue::CF7,
    });
    keymap[ScanCodeValue::F8 as usize] = Some(KeymapSet {
        not_shift: KeymapValue::F8,
        shift: KeymapValue::SF8,
        alt1: KeymapValue::AF8,
        alt2: KeymapValue::AF8,
        alt_shift: KeymapValue::ASF8,
        ctrl: KeymapValue::CF8,
    });
    keymap[ScanCodeValue::F9 as usize] = Some(KeymapSet {
        not_shift: KeymapValue::F9,
        shift: KeymapValue::SF9,
        alt1: KeymapValue::AF9,
        alt2: KeymapValue::AF9,
        alt_shift: KeymapValue::ASF9,
        ctrl: KeymapValue::CF9,
    });
    keymap[ScanCodeValue::F10 as usize] = Some(KeymapSet {
        not_shift: KeymapValue::F10,
        shift: KeymapValue::SF10,
        alt1: KeymapValue::AF10,
        alt2: KeymapValue::AF10,
        alt_shift: KeymapValue::ASF10,
        ctrl: KeymapValue::CF10,
    });
    keymap[ScanCodeValue::F11 as usize] = Some(KeymapSet {
        not_shift: KeymapValue::F11,
        shift: KeymapValue::SF11,
        alt1: KeymapValue::AF11,
        alt2: KeymapValue::AF11,
        alt_shift: KeymapValue::ASF11,
        ctrl: KeymapValue::CF11,
    });
    keymap[ScanCodeValue::F12 as usize] = Some(KeymapSet {
        not_shift: KeymapValue::F12,
        shift: KeymapValue::SF12,
        alt1: KeymapValue::AF12,
        alt2: KeymapValue::AF12,
        alt_shift: KeymapValue::ASF12,
        ctrl: KeymapValue::CF12,
    });
    keymap[ScanCodeValue::ScrollLock as usize] = Some(KeymapSet {
        not_shift: KeymapValue::ScrollLock,
        shift: KeymapValue::ScrollLock,
        alt1: KeymapValue::ScrollLock,
        alt2: KeymapValue::ScrollLock,
        alt_shift: KeymapValue::ScrollLock,
        ctrl: KeymapValue::ScrollLock,
    });
    keymap[ScanCodeValue::Insert as usize] = None;
    keymap[ScanCodeValue::Home as usize] = None;
    keymap[ScanCodeValue::PageUp as usize] = None;
    keymap[ScanCodeValue::Delete as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Delete,
        shift: KeymapValue::Delete,
        alt1: KeymapValue::AltDelete,
        alt2: KeymapValue::AltDelete,
        alt_shift: KeymapValue::AltDelete,
        ctrl: KeymapValue::Delete,
    });
    keymap[ScanCodeValue::End as usize] = None;
    keymap[ScanCodeValue::PageDown as usize] = None;
    keymap[ScanCodeValue::CursorRight as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Right,
        shift: KeymapValue::Right,
        alt1: KeymapValue::AltRight,
        alt2: KeymapValue::AltRight,
        alt_shift: KeymapValue::AltRight,
        ctrl: KeymapValue::ControlRight,
    });
    keymap[ScanCodeValue::CursorLeft as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Left,
        shift: KeymapValue::Left,
        alt1: KeymapValue::AltLeft,
        alt2: KeymapValue::AltLeft,
        alt_shift: KeymapValue::AltLeft,
        ctrl: KeymapValue::ControlLeft,
    });
    keymap[ScanCodeValue::CursorDown as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Down,
        shift: KeymapValue::Down,
        alt1: KeymapValue::AltDown,
        alt2: KeymapValue::AltDown,
        alt_shift: KeymapValue::AltDown,
        ctrl: KeymapValue::ControlDown,
    });
    keymap[ScanCodeValue::CursorUp as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Up,
        shift: KeymapValue::Up,
        alt1: KeymapValue::AltUp,
        alt2: KeymapValue::AltUp,
        alt_shift: KeymapValue::AltUp,
        ctrl: KeymapValue::ControlUp,
    });
    keymap[ScanCodeValue::NumberLock as usize] = Some(KeymapSet {
        not_shift: KeymapValue::NumberLock,
        shift: KeymapValue::NumberLock,
        alt1: KeymapValue::NumberLock,
        alt2: KeymapValue::NumberLock,
        alt_shift: KeymapValue::NumberLock,
        ctrl: KeymapValue::NumberLock,
    });
    keymap[ScanCodeValue::KeypadSlash as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Ascii('/'),
        shift: KeymapValue::Ascii('/'),
        alt1: KeymapValue::Alt('/'),
        alt2: KeymapValue::Alt('/'),
        alt_shift: KeymapValue::Alt('/'),
        ctrl: KeymapValue::Control('@'),
    });
    keymap[ScanCodeValue::KeypadStar as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Ascii('*'),
        shift: KeymapValue::Ascii('*'),
        alt1: KeymapValue::Alt('*'),
        alt2: KeymapValue::Alt('*'),
        alt_shift: KeymapValue::Alt('*'),
        ctrl: KeymapValue::Control('@'),
    });
    keymap[ScanCodeValue::KeypadMin as usize] = None;
    keymap[ScanCodeValue::KeypadPlus as usize] = None;
    keymap[ScanCodeValue::KeypadEnter as usize] = Some(KeymapSet {
        not_shift: KeymapValue::Control('M'),
        shift: KeymapValue::Control('M'),
        alt1: KeymapValue::ControlAlt('M'),
        alt2: KeymapValue::ControlAlt('M'),
        alt_shift: KeymapValue::ControlAlt('M'),
        ctrl: KeymapValue::Control('J'),
    });
    keymap[ScanCodeValue::Keypad1 as usize] = None;
    keymap[ScanCodeValue::Keypad2 as usize] = None;
    keymap[ScanCodeValue::Keypad3 as usize] = None;
    keymap[ScanCodeValue::Keypad4 as usize] = None;
    keymap[ScanCodeValue::Keypad5 as usize] = None;
    keymap[ScanCodeValue::Keypad6 as usize] = None;
    keymap[ScanCodeValue::Keypad7 as usize] = None;
    keymap[ScanCodeValue::Keypad8 as usize] = None;
    keymap[ScanCodeValue::Keypad9 as usize] = None;
    keymap[ScanCodeValue::Keypad0 as usize] = None;
    keymap[ScanCodeValue::KeypadDot as usize] = None;
    keymap[ScanCodeValue::LeftControl as usize] = Some(KeymapSet {
        not_shift: KeymapValue::LeftControl,
        shift: KeymapValue::LeftControl,
        alt1: KeymapValue::LeftControl,
        alt2: KeymapValue::LeftControl,
        alt_shift: KeymapValue::LeftControl,
        ctrl: KeymapValue::LeftControl,
    });
    keymap[ScanCodeValue::LeftShift as usize] = Some(KeymapSet {
        not_shift: KeymapValue::LeftShift,
        shift: KeymapValue::LeftShift,
        alt1: KeymapValue::LeftShift,
        alt2: KeymapValue::LeftShift,
        alt_shift: KeymapValue::LeftShift,
        ctrl: KeymapValue::LeftShift,
    });
    keymap[ScanCodeValue::LeftAlt as usize] = Some(KeymapSet {
        not_shift: KeymapValue::LeftAlt,
        shift: KeymapValue::LeftAlt,
        alt1: KeymapValue::LeftAlt,
        alt2: KeymapValue::LeftAlt,
        alt_shift: KeymapValue::LeftAlt,
        ctrl: KeymapValue::LeftAlt,
    });
    keymap[ScanCodeValue::LeftGUI as usize] = None;
    keymap[ScanCodeValue::RightControl as usize] = Some(KeymapSet {
        not_shift: KeymapValue::RightControl,
        shift: KeymapValue::RightControl,
        alt1: KeymapValue::RightControl,
        alt2: KeymapValue::RightControl,
        alt_shift: KeymapValue::RightControl,
        ctrl: KeymapValue::RightControl,
    });
    keymap[ScanCodeValue::RightShift as usize] = Some(KeymapSet {
        not_shift: KeymapValue::RightShift,
        shift: KeymapValue::RightShift,
        alt1: KeymapValue::RightShift,
        alt2: KeymapValue::RightShift,
        alt_shift: KeymapValue::RightShift,
        ctrl: KeymapValue::RightShift,
    });
    keymap[ScanCodeValue::RightAlt as usize] = Some(KeymapSet {
        not_shift: KeymapValue::RightAlt,
        shift: KeymapValue::RightAlt,
        alt1: KeymapValue::RightAlt,
        alt2: KeymapValue::RightAlt,
        alt_shift: KeymapValue::RightAlt,
        ctrl: KeymapValue::RightAlt,
    });
    keymap[ScanCodeValue::RightGUI as usize] = None;
    keymap
};
