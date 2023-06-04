# XLSX-to-CSV Converter written in Rust

The ultra-fast XLSX to CSV converter package, built leveraging the power of the Rust package - [Calamine](https://github.com/tafia/calamine). 
It outperforms [SheetJs](https://github.com/SheetJS/sheetjs), 
delivering **at least 3 times the speed** and **using only one-fifth of the memory**, making it ideal for processing large XLSX files.

## Installation

This package is available via NPM. To install, simply run the following command:

```
npm install xlsx-to-csv
```

## Usage

The package accepts both .xlsx and .csv file types. Here's a quick example illustrating the usage:

```typescript
import fs from 'fs'
import { convertFile } from 'xlsx-to-csv'

// Convert your file
const { filePath } = convertFile('./file.xlsx')

// Output the path of the converted file
console.log(filePath)

// Read the converted file
const csvData = fs.readFileSync(filePath).toString()

// Display the data
console.log(csvData)
```

## License

This project is licensed under the MIT License, providing a high degree of freedom for both personal and commercial use. For more details, please refer to the LICENSE file in the repository.
