# slint_redox_template

A  Slint (https://slint-ui.com/) app template for Redox (https://redox-os.org/).

## How to use

### Generate project

1. Install Rust by following the [Rust Getting Started Guide](https://www.rust-lang.org/learn/get-started).
   Once this is done, you should have the ```rustc``` compiler and the ```cargo``` build system installed in your path.
2. Install [`cargo-generate`](https://github.com/cargo-generate/cargo-generate)
    ```
    cargo install cargo-generate
    ```
3. Set up a sample project with this template
    ```
    cargo generate --git https://gitlab.redox-os.org/redox-os/slint_redox_template --name my-project
    cd my-project
    ```

### Run on Redox OS

#### Add your project as recipe to Redox

A description how to add your application to Redox as recipes can be found on the [Redox book](https://doc.redox-os.org/book/ch05-03-including-programs.html). An example [recipe.toml](recipe.toml) is include on this template.

#### Build redox with your app recipe

It's suggested to use podman to build the Redox with your application check the [README](https://gitlab.redox-os.org/redox-os/redox/-/blob/master/podman/README.md) for more details.

### Run on Linux, macOS, Windows

On other platforms then `Redox` by default Slint will select a suitable backend e.g. `winit` or `qt`. To run the app also on other platforms with `slint_orbclient` backend you can run it with `cargo run --bin my-project --no-default-features --features=orbclient`.

## License

The source code of the template is available under the terms of MIT license.
(See [LICENSE-MIT](LICENSE-MIT) for details.)

However, because of the use of GPL dependencies, the template, as a whole, is licensed
under the terms of the GPLv3 (See [LICENSE-GPL](LICENSE-GPL))
