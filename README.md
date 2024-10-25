# candlestick wasm
This reads a CSV file containing financial data and detects candlestick patterns, we'll need to follow several steps. We will implement the following:

1. **CSV File Handling**: Read and parse the CSV file.
2. **Candlestick Pattern Detection**: Implement a function to detect basic candlestick patterns (like "Bullish Engulfing", "Bearish Engulfing", etc.).
3. **Web Interface**: Allow users to upload a CSV file and display the detected patterns.

## Build the WebAssembly Module

Compile your Rust code into WebAssembly:

```bash
wasm-pack build --target web
```

## Run Your Application

1. Open your terminal and navigate to the project directory.
2. Use a local server to serve your files. For example, using Python's built-in server, run:

   ```bash
   python3 -m http.server
   ```

3. Open your web browser and go to `http://localhost:8000/index.html`.

## How It Works

- The user selects a CSV file containing candlestick data.
- When the "Detect Patterns" button is clicked, JavaScript reads the file content and calls the `detect_patterns` function from the Rust WebAssembly module.
- The function reads the candlestick data, detects patterns, and returns the results, which are displayed on the webpage.