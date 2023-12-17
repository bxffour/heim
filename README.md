# Heim - Simplifying SSH Connections

Heim is a Rust-based tool designed to streamline the process of connecting to your servers using SSH. It achieves this by parsing your SSH configuration file and presenting a list of hosts in an interactive terminal window. Once a host is selected, Heim automatically initiates an SSH connection.

## Installation

To build Heim, run the following command:

```bash
cargo build --release
```

## Usage

Heim takes a `--cfg_path` flag to specify the path to your SSH configuration file. Here's an example of how to use Heim:

```bash
heim --cfg_path /path/to/your/ssh/config
```

This will display an interactive terminal window containing a list of hosts parsed from the specified SSH configuration file. Simply select the desired host, and Heim will initiate an SSH connection.

## Demo

![Made with VHS](https://vhs.charm.sh/vhs-162Csuxh7Sm05ynEhD9ZT9.gif)

## Building Heim

To build Heim, use the following command:

```bash
cargo build --release
```

The `--release` flag ensures that the binary is optimized for performance.

## Contributing

If you'd like to contribute to Heim, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bug fix: `git checkout -b feature-name`.
3. Make your changes and commit them: `git commit -m "Your commit message here"`.
4. Push to the branch: `git push origin feature-name`.
5. Create a pull request.

## License

This project is licensed under the [MIT License](LICENSE).

## Acknowledgments

Heim is built with ❤️ by [Nana Kwadwo](https://github.com/bxffour) & [blvckprince](https://github.com/blackprince001).
