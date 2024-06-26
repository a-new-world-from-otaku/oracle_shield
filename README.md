# OracleShield

OracleShield is a solution developed to counteract Oracle's cloud computing resource reclamation program. It is written in Rust and uses the Tokio runtime.

## NOTE
This project is no longer being updated because there are extremely low-cost ways to counteract oracle cloud's reclamation strategy.

## Features

- Efficient command line argument parsing: OracleShield uses the `nom` crate to efficiently parse command line arguments for memory and frequency.
- High-performance calculations: OracleShield leverages the power of the Tokio runtime to perform asynchronous calculations of Pi.
- Robust signal handling: OracleShield uses a signal handler to gracefully handle Ctrl-C and ensure a clean shutdown.

## Usage

1. git clone
2. Run `cargo build --release` to build the program.
3. Run ./oracle_shield [option]

Available options include:

- `-m` or `--memory`: Specify the amount of memory to use (in GiB).
- `-p` or `--percent`: Specify the percent of CPU usage （as an integer between 0 and 100).

For example, to use 4 GiB of memory and 10%, run `./oracle_shield -m 4 -p 10`.

## Note

The portion implemented for CPU usage has not been tested in practice. If you encounter any issues, please feel free to submit a PR (Pull Request).

## Contributing

If you'd like to contribute to this project, please follow these steps:

1. Fork this repository.
2. Create a new branch.
3. Make your changes and commit them.
4. Push your changes to your forked repository.
5. Open a pull request.

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](./LICENSE) file for details.
