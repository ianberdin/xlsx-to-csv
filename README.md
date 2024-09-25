# The Fastest XLSX to CSV Converter. Rust powered.

This library provides an easy way to convert Excel files (`.xlsx`, `.xls`, `.xlsm`, etc.) to CSV format using custom
options such as specifying a sheet, delimiter, datetime format, and destination. It uses a Rust-powered backend for
efficient processing.

## Installation

To use this library, install the required NPM package by running:

```bash
# Using npm
npm install xlsx-to-csv

# Using yarn
yarn add xlsx-to-csv
```

Make sure to have the binary executable for the conversion process set up as per your platform requirements.

## Usage

### `convertFile(filePath: string, options: ConvertFileOptions = {}): string`

Converts an Excel file to CSV format with customizable options.

#### Parameters:

- `filePath` (`string`): Path to the Excel file you want to convert.

- `options` (`ConvertFileOptions`, optional): An object containing various settings to customize the conversion process.

### `ConvertFileOptions`

The options object allows you to configure the conversion with the following fields:

| Field            | Type     | Description                                                                                                                                                  | Default             |
|------------------|----------|--------------------------------------------------------------------------------------------------------------------------------------------------------------|---------------------|
| `sheet`          | `string` | Sheet name to convert. If not provided, the first sheet will be used.                                                                                        | First sheet         |
| `delimiter`      | `string` | Delimiter to use in the CSV file. You can choose between `;`, `,`, or any custom delimiter string.                                                           | `;`                 |
| `dest`           | `string` | Destination file path. If set to `'return'`, it will return the CSV data as a string. If not specified, it will print to `stdout`.                           | `stdout`            |
| `datetimeFormat` | `string` | Format for parsing datetime values in the sheet. Follows the [chrono format documentation](https://docs.rs/chrono/latest/chrono/format/strftime/index.html). | `%Y-%m-%d %H:%M:%S` |

### Example Usage

```typescript
import {convertFile} from 'your-library-name';

// Convert an Excel file to CSV and save it to a specific path
convertFile('example.xlsx', {
    sheet: 'Sheet1',
    delimiter: ',',
    dest: 'output.csv',
    datetimeFormat: '%d-%m-%Y %H:%M'
});

// Convert an Excel file to CSV and return the result as a string
const csvData = convertFile('example.xlsx', {
    sheet: 'Sheet1',
    delimiter: ';',
    dest: 'return',
    datetimeFormat: '%Y-%m-%d'
});

console.log(csvData);
```

### CLI Arguments Mapping

This library internally runs a binary command with the following mappings:

| Option           | CLI Argument                 |
|------------------|------------------------------|
| `filePath`       | `--file <filePath>`          |
| `sheet`          | `--sheet <sheet>`            |
| `delimiter`      | `--delimiter <delimiter>`    |
| `dest`           | `--dest <dest>`              |
| `datetimeFormat` | `--datetime-format <format>` |

### Additional Information

- The library supports a variety of Excel formats (`.xlsx`, `.xls`, `.xlsm`, `.xlsb`).
- For `datetimeFormat`, it uses the popular [chrono](https://docs.rs/chrono) crate for flexible and powerful date
  formatting options.

### Credits

- The maintainer is founder of [PlayCode.io](https://playcode.io) - an online code playground for web development.
- [Javascript Online Compiler](https://playcode.io/javascript-compiler) - Run your javascript code online.

### Contributing

- Install Node.js and NPM on your system.
- Clone the repository and install the dependencies using `yarn install`.
- Install Rust and Cargo on your system.
- Try running the project locally using `cargo build`.
- Prepare for cross-compilation by installing the required targets.
  - Windows: https://gist.github.com/Mefistophell/9787e1b6d2d9441c16d2ac79d6a505e6
  - Linux: `rustup target add x86_64-unknown-linux-gnu`
  - MacOS M*: `rustup target add aarch64-apple-darwin`
  - MacOS Intel: `rustup target add x86_64-apple-darwin`
  - Update vim ~/.cargo/config.toml with the following:
    ```toml
     [target.x86_64-unknown-linux-gnu]
     linker = "x86_64-unknown-linux-gnu-gcc"
  
     [target.x86_64-pc-windows-gnu]
     linker = "x86_64-w64-mingw32-gcc"
    ```

### License

This project is licensed under the MIT License.

---

Feel free to open an issue or contribute to the repository if you encounter any bugs or have feature suggestions!

