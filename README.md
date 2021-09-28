# xlsx-to-csv
Fastest xlsx to csv converter based on Rust package [Calamine](https://github.com/tafia/calamine). It faster than [SheetJs][https://github.com/SheetJS/sheetjs] at least 3 times and eats 5 times smaller memory,

### Install
`npm install xlsx-to-csv`

### Usage
It accepts .xlsx and .csv file types.
```typescript
import fs from 'fs'
import { convertFile } from 'xslx-to-csv'

const { filePath } = convertFile('./file.xlsx')
console.log(filePath)

const csvData = fs.readFileSync(filePath).toString()
console.log(csvData)
```

### Licence
MIT