mod hotkeys;

use std::rc::Rc;
use std::time::Duration;
use serialport::SerialPortType::{Unknown};
use slint::VecModel;
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

        let mut port = serialport::new(app_weak.unwrap().get_ports_combo_value().as_str(), 9600)
            .timeout(Duration::from_millis(10))
            .open()
            .expect("Failed to open port");

        port.write(message.as_ref()).expect("Failed to send Serial message");
    });

    app.set_window_title(SharedString::from("Pad Engine"));
    app.run()?;

    Ok(())
}