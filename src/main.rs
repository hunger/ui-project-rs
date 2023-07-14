use ui_project_rs::{add, div, mul, sub};

use slint::slint;

slint! {

import { Button, GridBox, VerticalBox} from "std-widgets.slint";

component Keyboard {
    callback key-pressed(string);

    GridBox {
        Row {
            Button { text: "7"; clicked => { root.key-pressed(self.text) }}
            Button { text: "8"; clicked => { root.key-pressed(self.text) }}
            Button { text: "9"; clicked => { root.key-pressed(self.text) }}
            Button { text: "Add"; clicked => { root.key-pressed(self.text) }}
            Button { rowspan: 4; text: "="; clicked => { root.key-pressed(self.text) }}
        }
        Row {
            Button { text: "4"; clicked => { root.key-pressed(self.text) }}
            Button { text: "5"; clicked => { root.key-pressed(self.text) }}
            Button { text: "6"; clicked => { root.key-pressed(self.text) }}
            Button { text: "Sub"; clicked => { root.key-pressed(self.text) }}
        }
        Row {
            Button { text: "1"; clicked => { root.key-pressed(self.text) }}
            Button { text: "2"; clicked => { root.key-pressed(self.text) }}
            Button { text: "3"; clicked => { root.key-pressed(self.text) }}
            Button { text: "Mul"; clicked => { root.key-pressed(self.text) }}
        }
        Row {
            Button { col: 2; text: "0"; clicked => { root.key-pressed(self.text) }}
            Button { col: 3; text: "Div"; clicked => { root.key-pressed(self.text) }}
        }
    }
}

export component AppWindow inherits Window {
    in-out property <int> current-value: 0;
    callback key-pressed(string);

    VerticalBox { 
        Text { text: root.current-value; }
        Keyboard { key-pressed(k) => { root.key_pressed(k) } }
    }
}

}

enum Operation {
    None,
    Add(i32),
    Sub(i32),
    Mul(i32),
    Div(i32),
}

fn main() -> Result<(), slint::PlatformError> {
    let app = AppWindow::new()?;

    let a = app.as_weak();
    let mut op = Operation::None;

    app.on_key_pressed(move |k: slint::SharedString| {
        let a = a.upgrade().unwrap();
        match k.as_bytes()[0] {
            c if c >= b'0' && c <= b'9' => {
                let current = a.get_current_value();
                let result = current * 10 + ((c - b'0') as i32);
                a.set_current_value(result);
            },
            b'A' => {
                op = Operation::Add(a.get_current_value());
                a.set_current_value(0);
            }
            b'S' => {
                op = Operation::Sub(a.get_current_value());
                a.set_current_value(0);
            }
            b'M' => {
                op = Operation::Mul(a.get_current_value());
                a.set_current_value(0);
            }
            b'D' => {
                op = Operation::Div(a.get_current_value());
                a.set_current_value(0);
            }
            b'=' => {
                let o2 = a.get_current_value();
                match op {
                    Operation::None => {},
                    Operation::Add(o1) => a.set_current_value(add(o1, o2)),
                    Operation::Sub(o1) => a.set_current_value(sub(o1, o2)),
                    Operation::Mul(o1) => a.set_current_value(mul(o1, o2)),
                    Operation::Div(o1) => a.set_current_value(div(o1, o2)),
                }
                op = Operation::None;
            }
            _ => todo!(),
        };
    });

    app.run()?;

    Ok(())
}
