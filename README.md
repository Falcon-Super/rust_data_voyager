# RustDataVoyage

RustDataVoyage is a Rust-based project designed to analyze CSV files and perform data validation and conversion. It uses the Rust programming language for its speed and safety, and can be containerized using Docker to ensure portability across different environments.

## Features

- CSV file validation
- Data type checks and conversions
- Exporting data to Apache Arrow format
- Docker support for easy deployment

## Prerequisites

Before you begin, ensure you have met the following requirements:

- Rust programming environment
- Docker (for containerization)
- Docker Compose

## Installation

To install RustDataVoyage, follow these steps:

```bash
git clone TODO
cd rustdatavoyage
cargo build --release
```

## Usage

To use RustDataVoyage, run the following command:

```bash
Copy code
cargo run -- input.csv output.arrow
```

Replace input.csv with the path to your CSV file, and output.arrow with the desired output file name for the Arrow format.

## Getting Started with Docker

### Building the Image

To build the Docker image for RustDataVoyage, run:

```bash
make build
```

#### Run the container

```bash
Copy code
docker run -v $(pwd):/data rustdatavoyage input.csv output.arrow
```

This command mounts the current directory to the container's /data directory, allowing the container to access input.csv and write output.arrow.

## Contributing

Contributions to RustDataVoyage are welcome. To contribute, please fork the repository, make your changes, and submit a pull request.

## License

This project is licensed under the MIT License.

## Contact

If you have any questions or feedback, please contact me via github.
