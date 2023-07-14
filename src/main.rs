use ui_project_rs::add;

use slint::slint;

slint! {
    export component AppWindow inherits Window {
        Text { text: "Hello World!"; }
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let result = add(4, 3);
    println!("Hello, world! 4 + 3 = {result}");

    let app = AppWindow::new()?;

    app.run()?;

    Ok(())
}
