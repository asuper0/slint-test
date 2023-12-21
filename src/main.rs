mod generated_code {
    slint::include_modules!();
}

pub use generated_code::*;

fn main() {
    let window = MainWindow::new().unwrap();
    window.run().unwrap();
}
