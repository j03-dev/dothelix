# dothelix

`dothelix` is a command-line tool for executing predefined tasks from a JSON configuration file. It reads tasks from `.helix/helix.json` and runs commands based on user input.

## Features

- Execute tasks defined in a JSON configuration file.
- Supports optional arguments and environment variables for each task.
- Simple and flexible way to automate common tasks like build and run.

## Installation

1. **Clone the repository**:
   ```sh
   git clone https://github.com/j03-dev/dothelix.git
   cd dothelix
   ```

2. **Build the project**:
   ```sh
   cargo build --release
   ```

3. **Install the executable**:
   ```sh
   cp target/release/dothelix /usr/local/bin/
   ```

   Alternatively, you can run the executable directly from the `target/release` directory:
   ```sh
   ./target/release/dothelix
   ```

## Usage

1. **Create a `.helix/helix.json` file** in your project root with the task definitions. Example:
   ```json
   {
       "run": {
           "command": "cargo",
           "args": ["run"],
           "env": {
               "RUST_LOG": "info"
           }
       },
       "build": {
           "command": "cargo",
           "args": ["build", "--release"]
       }
   }
   ```

2. **Run a task** by specifying its name:
   ```sh
   dothelix run
   ```

   or

   ```sh
   dothelix build
   ```
3. **Open `~/.config/helix/config.toml` and add this:
  ```toml
  [keys.normal]
  F5 = [":run-shell-command dothelix build"]
  F9 = [":run-shell-command dothelix build"]
  ```
## Configuration

The `.helix/helix.json` file should be located in the `.helix` directory in your project root. The structure is as follows:

- `run` and `build` are task names. You can define more if needed.
- `command` is the base command to execute.
- `args` is an optional array of command-line arguments.
- `env` is an optional object of environment variables.

### Example

```json
{
    "run": {
        "command": "node",
        "args": ["index.js"],
        "env": {
            "NODE_ENV": "development"
        }
    },
    "build": {
        "command": "cargo",
        "args": ["build"],
        "env": {
            "RUST_LOG": "debug"
        }
    }
}
```
