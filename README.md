# Realtime Options Ticker

This project is a real-time stock options ticker application that fetches and displays live options data. It is built using Rust and leverages asynchronous programming for efficient data handling.

## Features

- Fetches real-time options data from a chosen API (e.g., Alpha Vantage or IEX Cloud).
- Displays options data through both a command-line interface and a web-based user interface.
- Supports real-time updates via WebSocket connections.
- Processes and calculates metrics for options data.

## Project Structure

```
realtime-options-ticker
├── src
│   ├── main.rs          # Entry point of the application
│   ├── lib.rs           # Library code for main functionalities
│   ├── api              # Module for API interactions
│   │   └── mod.rs       # API functions to fetch and parse options data
│   ├── models           # Module for data structures
│   │   └── mod.rs       # Defines OptionData and OptionQuote structs
│   ├── services         # Module for business logic
│   │   └── mod.rs       # Functions to process options data
│   ├── ui               # Module for user interfaces
│   │   ├── terminal.rs   # Command-line interface implementation
│   │   └── web.rs       # Web-based user interface implementation
│   ├── ws.rs            # WebSocket handling for real-time updates
│   └── errors.rs        # Custom error types and handling
├── Cargo.toml           # Project configuration and dependencies
├── .gitignore           # Files and directories to ignore by Git
└── README.md            # Project documentation
```

## Setup Instructions

1. Clone the repository:
   ```
   git clone <repository-url>
   cd realtime-options-ticker
   ```

2. Install Rust and Cargo if you haven't already. Follow the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).

3. Build the project:
   ```
   cargo build
   ```

4. Run the application:
   ```
   cargo run
   ```

## Usage Examples

- To start the command-line interface, run the application and follow the prompts.
- For the web interface, navigate to `http://localhost:8000` after starting the server.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any enhancements or bug fixes.