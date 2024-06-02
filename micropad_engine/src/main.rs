mod hotkeys;

use std::rc::Rc;
use std::time::Duration;
use serde_json::Value;
use serialport::SerialPort;
use serialport::SerialPortType::{Unknown};
use slint::{VecModel, Weak};
use slint::SharedString;
slint::include_modules!();
// fn main() {
//     //
//     // let mut port = serialport::new("/dev/ttyACM1", 9600)
//     //     .timeout(Duration::from_millis(10))
//     //     .open()
//     //     .expect("Failed to open port");
//     //
//     // port.write(b"test");
// }
fn get_ports() -> Vec<SharedString> {
    let mut ports: Vec<SharedString> = vec![];
    let serial_ports = serialport::available_ports().expect("No ports found!");

    for p in serial_ports {
        if p.port_type != Unknown {
            ports.push(SharedString::from(p.port_name));
        }
    }

    ports
}

fn write_hotkeys(app_weak: &Weak<AppWindow>, hotkeys: &Value) -> String {
    let box1_1: usize = app_weak.unwrap().get_box1_1_current_index() as usize;
    let box1_2: usize = app_weak.unwrap().get_box1_2_current_index() as usize;
    let box1_3: usize = app_weak.unwrap().get_box1_3_current_index() as usize;
    let box2_1: usize = app_weak.unwrap().get_box2_1_current_index() as usize;
    let box2_2: usize = app_weak.unwrap().get_box2_2_current_index() as usize;
    let box2_3: usize = app_weak.unwrap().get_box2_3_current_index() as usize;
    let box3_1: usize = app_weak.unwrap().get_box3_1_current_index() as usize;
    let box3_2: usize = app_weak.unwrap().get_box3_2_current_index() as usize;
    let box3_3: usize = app_weak.unwrap().get_box3_3_current_index() as usize;

    // convert value to key
    let box1_1 = hotkeys::find_key_by_index(&hotkeys, box1_1).unwrap();
    let box1_2 = hotkeys::find_key_by_index(&hotkeys, box1_2).unwrap();
    let box1_3 = hotkeys::find_key_by_index(&hotkeys, box1_3).unwrap();
    let box2_1 = hotkeys::find_key_by_index(&hotkeys, box2_1).unwrap();
    let box2_2 = hotkeys::find_key_by_index(&hotkeys, box2_2).unwrap();
    let box2_3 = hotkeys::find_key_by_index(&hotkeys, box2_3).unwrap();
    let box3_1 = hotkeys::find_key_by_index(&hotkeys, box3_1).unwrap();
    let box3_2 = hotkeys::find_key_by_index(&hotkeys, box3_2).unwrap();
    let box3_3 = hotkeys::find_key_by_index(&hotkeys, box3_3).unwrap();

    let mut message: String = String::new();
    let mut message_b1: String = String::from("wh;1,press");
    let mut message_b2: String = String::from("2,press");
    let mut message_b3: String = String::from("3,press");

    for key in [box1_1, box1_2, box1_3] {
        if key != "none" {
            message_b1.push_str(&format!(",{}", key));
        }
    }
    for key in [box2_1, box2_2, box2_3] {
        if key != "none" {
            message_b2.push_str(&format!(",{}", key));
        }
    }
    for key in [box3_1, box3_2, box3_3] {
        if key != "none" {
            message_b3.push_str(&format!(",{}", key));
        }
    }
    message.push_str(&format!("{};{};{}", message_b1, message_b2, message_b3));
    println!("{}", message);

    message
}

fn write_backlight(app_weak: &Weak<AppWindow>) -> String {
    // let message: String = String::from("wl;1,0,86,222");
    let lineEdit1_color_red_value: SharedString = app_weak.unwrap().get_lineEdit1_color_red_value();
    let lineEdit1_color_green_value: SharedString = app_weak.unwrap().get_lineEdit1_color_green_value();
    let lineEdit1_color_blue_value: SharedString = app_weak.unwrap().get_lineEdit1_color_blue_value();

    let lineEdit2_color_red_value: SharedString = app_weak.unwrap().get_lineEdit2_color_red_value();
    let lineEdit2_color_green_value: SharedString = app_weak.unwrap().get_lineEdit2_color_green_value();
    let lineEdit2_color_blue_value: SharedString = app_weak.unwrap().get_lineEdit2_color_blue_value();

    let lineEdit3_color_red_value: SharedString = app_weak.unwrap().get_lineEdit3_color_red_value();
    let lineEdit3_color_green_value: SharedString = app_weak.unwrap().get_lineEdit3_color_green_value();
    let lineEdit3_color_blue_value: SharedString = app_weak.unwrap().get_lineEdit3_color_blue_value();

    let mut message: String = String::new();
    let mut message_b1: String = String::from("wl;1");
    let mut message_b2: String = String::from("2");
    let mut message_b3: String = String::from("3");


    for value in [lineEdit1_color_red_value, lineEdit1_color_green_value, lineEdit1_color_blue_value] {
        if value != "" {
            message_b1.push_str(&format!(",{}", value));
        }
    }
    for value in [lineEdit2_color_red_value, lineEdit2_color_green_value, lineEdit2_color_blue_value] {
        if value != "" {
            message_b2.push_str(&format!(",{}", value));
        }
    }
    for value in [lineEdit3_color_red_value, lineEdit3_color_green_value, lineEdit3_color_blue_value] {
        if value != "" {
            message_b3.push_str(&format!(",{}", value));
        }
    }

    message.push_str(&format!("{};{};{}", message_b1, message_b2, message_b3));
    println!("{}", message);

    message
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let hotkeys = hotkeys::get_json();

    let app = AppWindow::new()?;

    let ports = Rc::from(VecModel::from(get_ports()));

    let mut hotkeys2: Vec<SharedString> = vec![];
    for value in hotkeys::get_values(&hotkeys) {
        hotkeys2.push(SharedString::from(value));
    }

    let hotkeys3 = Rc::from(VecModel::from(hotkeys2.clone()));

    app.set_ports_combo_model(ports.into());
    app.set_actions(hotkeys3.into());

    // callbacks
    let app_weak = app.as_weak();
    app.on_cl(move || {
        let mut message: String = String::new();
        let mut port = serialport::new(app_weak.unwrap().get_ports_combo_value().as_str(), 9600)
            .timeout(Duration::from_millis(10))
            .open()
            .expect("Failed to open port");

        let tab_index: i32 = app_weak.unwrap().get_tab_current_index();

        match tab_index {
            0 => message = String::from(write_hotkeys(&app_weak, &hotkeys)),
            1 => message = String::from(write_backlight(&app_weak)),
            _ => {}
        }

        if !message.is_empty() {
            port.write(message.as_ref()).expect("Failed to send Serial message");
        }
    });

    app.set_window_title(SharedString::from("Pad Engine"));
    app.run()?;

    Ok(())
}