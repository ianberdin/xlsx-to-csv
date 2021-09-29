use calamine::{open_workbook_auto, DataType, Range, Reader};
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

fn main() {
    let file = env::args()
        .nth(1)
        .expect("Please provide an excel file to convert");

    let mut sheet = env::args().nth(2);

    let sce = PathBuf::from(file);
    match sce.extension().and_then(|s| s.to_str()) {
        Some("xlsx") | Some("xlsm") | Some("xlsb") | Some("xls") => (),
        _ => panic!("Expecting an excel file"),
    }

    let dest = sce.with_extension("csv");
    let mut dest = BufWriter::new(File::create(dest).unwrap());
    let mut xl = open_workbook_auto(&sce).unwrap();

    if sheet.is_none() {
        sheet = xl.sheet_names().first().cloned();
    }

    let range = xl.worksheet_range(&sheet.unwrap()).unwrap().unwrap();

    write_range(&mut dest, &range).unwrap();
}

fn write_range<W: Write>(dest: &mut W, range: &Range<DataType>) -> std::io::Result<()> {
    let rows_height = range.get_size().1 - 1;

    for (_row_index, row) in range.rows().enumerate() {
        for (cell_index, cell) in row.iter().enumerate() {
            match *cell {
                DataType::Empty => Ok(()),
                DataType::String(ref s) => write!(dest, "{}", s),
                DataType::Float(ref f) => write!(dest, "{}", f),
                DataType::DateTime(ref _f) => {

                    // Format as DD/MM/YY
                    write!(dest, "{}", DataType::as_date(cell).unwrap().format("%d/%m/%y"))
                }
                DataType::Int(ref i) => write!(dest, "{}", i),
                DataType::Error(ref e) => write!(dest, "{:?}", e),
                DataType::Bool(ref b) => write!(dest, "{}", b),
            }?;
            if cell_index != rows_height {
                write!(dest, ";")?;
            }
        }
        write!(dest, "\r\n")?;
    }
    Ok(())
}