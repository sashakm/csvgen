const DEFAULT_FLOAT: f64 = 3.1415;
const DEFAULT_SCHTRING: &str = "test";
const DEFAULT_INT: usize = 42;

enum CellType {
    /// Supported Types for column-cells
    Float,
    Schtring,
    Int
}

struct Cell {
    /// Represents one cell in a row
    cell_value: String,
}

pub struct CsvLine {
    /// Represents one row of csv consisting of multiple Cells with CellTypes
    pub line_value: String
}

impl Cell {
    /// Populate cell according to given CellType
    /// ```
    /// use csvgen::generator::{Cell,CellType};
    /// let c_type = CellType::Int;
    /// let c = Cell::new(c_type);
    /// ```
    pub fn new(c_type: CellType) -> Cell {
        Cell {
            cell_value: match c_type {
                CellType::Float => DEFAULT_FLOAT.to_string(),
                CellType::Schtring => String::from(DEFAULT_SCHTRING),
                CellType::Int => DEFAULT_INT.to_string(),
            }
        }
    } 
}

impl CsvLine {
    /// Create a csv-row according to given types.
    /// For each type, a Cell will be generated. The Cell-value 
    /// is then pushed onto a String held by CsvLine.
    /// ```
    /// use csvgen::generator::CsvLine;
    /// let line_types = Vec::from(vec!["int","string"]);
    /// let line = CsvLine::new(&line_types);
    /// ```
    pub fn new(l_types: &Vec<String>) -> CsvLine {
        let mut val: String = String::new();
        for (cell_num, lt) in l_types.iter().enumerate() {
            let t: CellType = match lt.as_ref() {
                "string" => CellType::Schtring,
                "int" => CellType::Int,
                "float" => CellType::Float,
                _ => panic!("Failed to determine cell type.")
            };
            let c = Cell::new(t);
            val.push_str(&c.cell_value);
            if cell_num < l_types.len()-1 {
                val.push_str(",")
            } else {
                val.push_str("\n")
            }
        }
        CsvLine {
            line_value: val
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_csv_line_generation() {
        use generator::{CsvLine,DEFAULT_FLOAT,DEFAULT_INT,DEFAULT_SCHTRING};

        let test_types = Vec::from(vec!["string".to_owned(),"int".to_owned(),"float".to_owned()]);
        let test_line = CsvLine::new(&test_types);
        let validate_types = format!("{},{},{}\n", &DEFAULT_SCHTRING
                                               , &DEFAULT_INT
                                               , &DEFAULT_FLOAT);
        assert_eq!(test_line.line_value,validate_types);
    }
}