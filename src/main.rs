use ui_project_rs::add;

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

fn main() -> Result<(), slint::PlatformError> {
    let app = AppWindow::new()?;

    app.on_key_pressed(|k: slint::SharedString| {
        eprintln!("Key pressed: {k}");
    });

    app.run()?;

    Ok(())
}
