use calamine::{open_workbook_auto, Data, Range, Reader};
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Source xlsx file
    #[arg(short, long)]
    file: String,

    /// Sheet name to convert
    /// If not provided, the first sheet will be used
    #[arg(short, long)]
    sheet: Option<String>,

    /// Delimiter to use in the CSV file
    #[arg(short, long, default_value = ";")]
    delimiter: String,

    /// Datetime format to use (e.g., "%Y-%m-%d %H:%M:%S")
    #[arg(short = 'f', long, default_value = "%Y-%m-%d %H:%M:%S")]
    datetime_format: String,

    /// Output file path (use "stdout" for printing to standard output)
    #[arg(short, long, default_value = "stdout")]
    dest: String,
}

fn main() {
    let args = Args::parse();
    let file: String = args.file;
    let mut sheet: Option<String> = args.sheet;
    let delimiter: String = args.delimiter;
    let date_format: String = args.datetime_format;
    let output = args.dest;

    let sce = PathBuf::from(file);
    match sce.extension().and_then(|s| s.to_str()) {
        Some("xlsx") | Some("xlsm") | Some("xlsb") | Some("xls") => (),
        _ => panic!("Expecting an excel file"),
    }

    let mut xl = open_workbook_auto(&sce).unwrap();

    if sheet.is_none() {
        sheet = xl.sheet_names().first().cloned();
    }

    // Open output as either a file or stdout
    let mut dest: Box<dyn Write> = if output == "stdout" {
        Box::new(io::stdout())
    } else {
        Box::new(BufWriter::new(File::create(PathBuf::from(output)).unwrap()))
    };

    let range = xl.worksheet_range(&sheet.unwrap()).unwrap();

    // Pass the delimiter and date format to the write_range function
    write_range(&mut dest, &range, &delimiter, &date_format).unwrap();
}

fn write_range<W: Write>(
    dest: &mut W,
    range: &Range<Data>,
    delimiter: &str,
    datetime_format: &str,
) -> std::io::Result<()> {
    let rows_height = range.get_size().1 - 1;

    for (_row_index, row) in range.rows().enumerate() {
        for (cell_index, cell) in row.iter().enumerate() {
            // Prepare the value to be written
            let mut value = match *cell {
                Data::Empty => String::new(),
                Data::String(ref s) | Data::DateTimeIso(ref s) | Data::DurationIso(ref s) => s.clone(),
                Data::Float(ref f) => f.to_string(),
                Data::DateTime(ref d) => d.as_datetime().unwrap().format(datetime_format).to_string(),
                Data::Int(ref i) => i.to_string(),
                Data::Error(ref e) => format!("{:?}", e),
                Data::Bool(ref b) => b.to_string(),
            };

            // Check if the value contains the delimiter or quotes
            if value.contains(delimiter) || value.contains('"') {
                // Escape existing double quotes
                value = value.replace("\"", "\"\"");
                // Enclose the value in double quotes
                value = format!("\"{}\"", value);
            }

            // Write the value to the output
            write!(dest, "{}", value)?;

            // Use the provided delimiter instead of hardcoded ","
            if cell_index != rows_height {
                write!(dest, "{}", delimiter)?;
            }
        }
        write!(dest, "\r\n")?;
    }
    Ok(())
}
