#[allow(clippy::all)]
mod generated_code {
    slint::include_modules!();
}

pub use generated_code::*;

pub fn main() {
    let app = App::new().unwrap();

    app.run().unwrap();
}
