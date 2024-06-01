use crate::hotkeys;
use serde_json::*;
pub const HOTKEYS: &str = r#"{
  "none": "[NO ACTION]",
  "a": "a",
  "b": "b",
  "c": "c",
  "d": "d",
  "e": "e",
  "f": "f",
  "g": "g",
  "h": "h",
  "i": "i",
  "j": "j",
  "k": "k",
  "l": "l",
  "m": "m",
  "n": "n",
  "o": "o",
  "p": "p",
  "q": "q",
  "r": "r",
  "s": "s",
  "t": "t",
  "u": "u",
  "v": "v",
  "w": "w",
  "x": "x",
  "y": "y",
  "z": "z",
  "WIN": "0x83",
  "LEFT CTRL": "0x80",
  "LEFT SHIFT": "0x81",
  "LEFT ALT": "0x82",
  "RIGHT CTRL": "0x84",
  "RIGHT SHIFT": "0x85",
  "RIGHT ALT": "0x86",
  "RIGHT WIN": "0x87",
  "RETURN": "0xB0",
  "UP": "0xDA",
  "DOWN": "0xD9",
  "LEFT": "0xD8",
  "RIGHT": "0xD7",
  "BACKSPACE": "0xB2",
  "TAB": "0xB3",
  "MENU": "0xED",
  "ESC": "0xB1",
  "INSERT": "0xD1",
  "DELETE": "0xD4",
  "PAGE UP": "0xD3",
  "PAGE DOWN": "0xD6",
  "HOME": "0xD2",
  "END": "0xD5",
  "CAPS LOCK": "0xC1",
  "PRINT SCREEN": "0xCE",
  "SCROLL LOCK": "0xCF",
  "PAUSE": "0xD0",
  "NUM LOCK": "0xDB",
  "KP_SLASH": "0xDC",
  "KP_ASTERISK": "0xDD",
  "KP_MINUS": "0xDE",
  "KP_PLUS": "0xDF",
  "KP_ENTER": "0xE0",
  "": "",
  "": "",
  "": "",
  "": "",
  "": "",
  "": "",
  "": "",
  "": "",

}"#;
pub fn get_json() -> serde_json::Value {
    let json_data: serde_json::Value = serde_json::from_str(&HOTKEYS)
        .expect("Can't parse json");
    json_data
}
pub fn get_values(json_data: &Value) -> Vec<String> {
    let values: Vec<&serde_json::Value> = json_data.as_object().unwrap().values().collect();
    let string_values: Vec<String> = values.iter().map(|v| v.as_str().unwrap_or_default().to_string()).collect();
    string_values
}

pub fn find_key_by_index(json: &Value, target_index: usize) -> Option<String> {
    if let Value::Object(map) = json {
        // Получаем итератор по ключам и значениям JSON объекта
        for (index, (key, _value)) in map.iter().enumerate() {
            if index == target_index {
                return Some(key.clone());
            }
        }
    }
    None
}