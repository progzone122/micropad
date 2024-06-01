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
  "0x83": "WIN",
  "0x80": "LEFT CTRL",
  "0x81": "LEFT SHIFT",
  "0x82": "LEFT ALT",
  "0x84": "RIGHT CTRL",
  "0x85": "RIGHT SHIFT",
  "0x86": "RIGHT ALT",
  "0x87": "RIGHT WIN",
  "0xB0": "RETURN",
  "0xDA": "UP",
  "0xD9": "DOWN",
  "0xD8": "LEFT",
  "0xD7": "RIGHT",
  "0xB2": "BACKSPACE",
  "0xB3": "TAB",
  "0xED": "MENU",
  "0xB1": "ESC",
  "0xD1": "INSERT",
  "0xD4": "DELETE",
  "0xD3": "PAGE UP",
  "0xD6": "PAGE DOWN",
  "0xD2": "HOME",
  "0xD5": "END",
  "0xC1": "CAPS LOCK",
  "0xCE": "PRINT SCREEN",
  "0xCF": "SCROLL LOCK",
  "0xD0": "PAUSE",
  "0xDB": "NUM LOCK",
  "0xDC": "KP_SLASH",
  "0xDD": "KP_ASTERISK",
  "0xDE": "KP_MINUS",
  "0xDF": "KP_PLUS",
  "0xE0": "KP_ENTER"
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