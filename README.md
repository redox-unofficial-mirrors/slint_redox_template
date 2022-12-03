# slint_redox_template

A template Slint (https://slint-ui.com/) template for Redox (https://redox-os.org/).

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
    cargo generate --git https://github.com/slint-ui/slint-rust-template --name my-project
    cd my-project
    ```

### Run on Redox OS

#### Add your project as recipe to Redox

A description how to add your application to Redox as recipes can be found on the [Redox book](https://doc.redox-os.org/book/ch05-03-including-programs.html). An example [recipe.toml](recipe.toml) is include on this template.

#### Build redox with your app recipe

It's suggested to use podman to build the Redox with your application check the [README](https://gitlab.redox-os.org/redox-os/redox/-/blob/master/podman/README.md) for more details. It's important to add the package `libfontconfig1-dev` to the podman [Container file](https://gitlab.redox-os.org/redox-os/redox/-/blob/master/podman/redox-base-containerfile). In the readme is also described how to rebuild the container.