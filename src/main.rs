#[allow(clippy::all)]
mod generated_code {
    slint::include_modules!();
}

pub use generated_code::*;

fn eval(input: &str) -> String {
    match calc::eval(input) {
        Ok(s) => s.to_string(),
        Err(e) => e.into(),
    }
}

pub fn main() {
    #[cfg(any(target_os = "redox", feature = "orbclient"))]
    slint_orbclient::init_config(
        slint_orbclient::Config::default()
            .width(600)
            .height(400)
            .resizable(true)
            .events_async(true)
            .title("My slint app"),
    );

    let app = App::new();

    app.on_backspace(|input| {
        let mut input = input.to_string();
        input.pop();
        input.into()
    });

    app.on_calculate(|input| eval(input.as_str()).into());

    app.run();
}
