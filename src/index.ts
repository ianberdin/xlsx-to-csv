import * as binary from './get-binary'

interface ConvertFileOptions {
    // Sheet name to convert
    // Default first sheet
    sheet?: string

    // Output delimiter for CSV file
    // Default is ';'
    delimiter?: ',' | ';' | string

    // Destination file path or 'return' to return the CSV data
    // Default is 'stdout'
    dest?: string | 'return'

    // Datetime format for parsing datetime values
    // See available options: https://docs.rs/chrono/latest/chrono/format/strftime/index.html
    // Default is '%Y-%m-%d %H:%M:%S'
    datetimeFormat?: string
}

export function convertFile(filePath: string, options: ConvertFileOptions = {}): string {
    const args = [
        '--file', filePath,
    ]

    if (options.sheet) {
        args.push('--sheet', options.sheet)
    }
    if (options.delimiter) {
        args.push('--delimiter', options.delimiter)
    }

    if (options.datetimeFormat) {
        args.push('--datetime-format', options.datetimeFormat)
    }

    if (options.dest) {
        if (options.dest === 'return') {
            args.push('--dest', 'stdout')
        } else {
            args.push('--dest', options.dest)
        }
    }

    return binary.run(...args).output.toString()
}

export default {
    convertFile,
}