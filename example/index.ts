import {convertFile} from '../dist'

// Convert your file
const result = convertFile('./example/file-header.xlsx',
    {
        delimiter: ',',

        dest: 'return',
        // dest: 'example/output.csv',

        // See available options: https://docs.rs/chrono/latest/chrono/format/strftime/index.html
        datetimeFormat: '%Y-%m-%d'
    }
)

// Output the path of the converted file
console.log(result)