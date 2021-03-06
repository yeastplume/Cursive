use event::{Event, Key};
use std::collections::HashMap;
use theme::{BaseColor, Color};

#[cfg(feature = "ncurses")]
mod n;
#[cfg(feature = "ncurses")]
pub use self::n::*;

#[cfg(feature = "pancurses")]
mod pan;
#[cfg(feature = "pancurses")]
pub use self::pan::*;

fn split_i32(code: i32) -> Vec<u8> {
    (0..4).map(|i| ((code >> (8 * i)) & 0xFF) as u8).collect()
}

fn fill_key_codes<F>(target: &mut HashMap<i32, Event>, f: F)
where
    F: Fn(i32) -> Option<String>,
{
    let key_names = hashmap!{
        "DC" => Key::Del,
        "DN" => Key::Down,
        "END" => Key::End,
        "HOM" => Key::Home,
        "IC" => Key::Ins,
        "LFT" => Key::Left,
        "NXT" => Key::PageDown,
        "PRV" => Key::PageUp,
        "RIT" => Key::Right,
        "UP" => Key::Up,
    };

    for code in 512..1024 {
        let name = match f(code) {
            Some(name) => name,
            None => continue,
        };

        if !name.starts_with('k') {
            continue;
        }

        let (key_name, modifier) = name[1..].split_at(name.len() - 2);
        let key = match key_names.get(key_name) {
            Some(&key) => key,
            None => continue,
        };
        let event = match modifier {
            "3" => Event::Alt(key),
            "4" => Event::AltShift(key),
            "5" => Event::Ctrl(key),
            "6" => Event::CtrlShift(key),
            "7" => Event::CtrlAlt(key),
            _ => continue,
        };
        target.insert(code, event);
    }
}

fn find_closest(color: &Color) -> i16 {
    match *color {
        Color::TerminalDefault => -1,
        Color::Dark(BaseColor::Black) => 0,
        Color::Dark(BaseColor::Red) => 1,
        Color::Dark(BaseColor::Green) => 2,
        Color::Dark(BaseColor::Yellow) => 3,
        Color::Dark(BaseColor::Blue) => 4,
        Color::Dark(BaseColor::Magenta) => 5,
        Color::Dark(BaseColor::Cyan) => 6,
        Color::Dark(BaseColor::White) => 7,
        Color::Light(BaseColor::Black) => 8,
        Color::Light(BaseColor::Red) => 9,
        Color::Light(BaseColor::Green) => 10,
        Color::Light(BaseColor::Yellow) => 11,
        Color::Light(BaseColor::Blue) => 12,
        Color::Light(BaseColor::Magenta) => 13,
        Color::Light(BaseColor::Cyan) => 14,
        Color::Light(BaseColor::White) => 15,
        Color::Rgb(r, g, b) => {
            // If r = g = b, it may be a grayscale value!
            if r == g && g == b && r != 0 && r < 250 {
                // (r = g = b) = 8 + 10 * n
                // (r - 8) / 10 = n
                //
                let n = (r - 8) / 10;
                (232 + n) as i16
            } else {
                let r = 6 * u16::from(r) / 256;
                let g = 6 * u16::from(g) / 256;
                let b = 6 * u16::from(b) / 256;
                (16 + 36 * r + 6 * g + b) as i16
            }
        }
        Color::RgbLowRes(r, g, b) => i16::from(16 + 36 * r + 6 * g + b),
    }
}
