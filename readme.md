## Log File Parsing and Export Utility

**Purpose:**  
This tool is a CLI-based Rust application designed to parse log files, extract key information, and export the processed data into a structured CSV format. It is tailored for applications requiring efficient log analysis and data transformation for further analysis or archival purposes.

---

### Key Features

1. **Flexible Log Parsing**
    - Parses `.log` and `.txt` files to extract key data points such as:
        - `SiteNo`
        - `Max_Temp`
        - `Temperature Differential`
        - `Timestamp`
        - `Diag` information.

2. **Customizable Output**
    - Generates CSV files with a predefined structure:
        - Columns: `datetime`, `site`, `max_temp`, `temperature_differential`, and `diag`.

3. **High Performance**
    - Efficiently handles large files using buffered input and minimal memory overhead.
    - Implements progress tracking to monitor the processing status in real time.

4. **Error Handling**
    - Ensures user-friendly error messages for invalid file paths.
    - Validates input log format to prevent data corruption.

5. **CLI Integration**
    - Supports file input via command-line arguments.
    - Processes multiple files in a single run.

---

### Prerequisites

- **Rust Compiler**: Ensure Rust is installed. You can download it from [rust-lang.org](https://www.rust-lang.org/).
- **Input Files**: Log files (`.log` or `.txt`) must follow a specific format containing recognizable markers like `@JSON@` and `@param@`.

---

### Usage Instructions

1. **Build the Application**
    - Clone the repository and navigate to the project directory:
      ```bash
      git clone https://github.com/stop1204/cobra_log_collect.git
      cd log-parser
      ```
    - Build the executable using Cargo:
      ```bash
      cargo build --release
      ```

2. **Run the Application**
    - Execute the program, specifying log files as arguments:
      ```bash
      ./target/release/log-parser file1.log file2.txt
      ```
    - The program will generate `.csv` output files in the same directory as the input files.

3. **Interactive File Input**
    - If no arguments are provided, the program will prompt for a file path:
      ```text
      please input file path:
      ```
    - Enter the absolute path to the desired log file.

4. **Output Structure**
    - The resulting CSV file will be named as `<original_filename>.csv` and contain structured data in the following format:
      ```csv
      datetime,site,max_temp,temperature_differential,diag
      "2024-04-23 10:03:53",13,52.0,37.2,"Cooling Issue"
      ```

---

### Example Workflow

**Input File Example (`example.log`):**
```text
@JSON@  "SiteNo" :  13,
@param@ Max_Temp 52.0
@JSON@  "timestamp" :  "Tue Apr 23 10:03:53 2024",
@param@ Temperature_Differential 37.2
Diag: Cooling Issue
```

**Execution Command:**
```bash
./target/release/log-parser example.log
```

**Output File Example (`example.log.csv`):**
```csv
datetime,site,max_temp,temperature_differential,diag
2024-04-23 10:03:53,13,52.0,37.2,Cooling Issue
```

---

### Error Handling

- **File Not Found**:
    - If the specified file does not exist, the program will output:
      ```text
      file not found
      will exit in 5 seconds
      ```
- **Incomplete Data**:
    - If a `SiteNo` is detected without accompanying `Max_Temp`, the entry will be skipped.

---

### Technical Highlights

1. **ANSI-Based Progress Tracking**
    - Utilizes ANSI escape codes to display real-time progress updates during log processing.

2. **Custom Parsing Logic**
    - Detects specific markers like `@JSON@` and `@param@` for structured data extraction.
    - Handles multi-line records and nested information efficiently.

3. **Thread-Safe Design**
    - Employs scoped threads for concurrent file handling without data race conditions.

4. **Extensibility**
    - Modular architecture allows for easy integration of additional data extraction rules or export formats.

---

### Future Enhancements

- Support for additional file formats (e.g., JSON, XML).
- Enhanced error recovery for malformed logs.
- Integration with cloud storage for automated uploads.

---

### Contribution

Contributions to improve the parser's functionality or add new features are welcome. Fork the repository, make changes, and submit a pull request.

**Contact**: For issues or inquiries, please open a ticket in the repository's issue tracker.

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
