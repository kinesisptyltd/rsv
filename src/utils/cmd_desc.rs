pub const COUNT_DESC: &str = "Count the number of lines. The command deals with raw bytes
and is therefore fast. When supplemented with a directory, the command count the number of
files in the directory.

Usage: 
  rsv.exe count [OPTIONS] <FILENAME>
  rsv count data.csv
  rsv count --no-header data.csv
  rsv count directory

Arguments:
  <FILENAME>  File to open

Options:
      --no-header  Whether the file has a header
  -h, --help       Print help information (use `--help` for more detail)
";

pub const HEADER_DESC: &str = "Show file headers

Usage: 
  rsv.exe headers [OPTIONS] <FILENAME>
  rsv headers data.csv
  rsv headers -s \\t data.csv

Arguments:
  <FILENAME>  File to open

Options:
  -s, --sep <SEP>  Field separator [default: ,]
  -h, --help       Print help information";

pub const HEAD_DESC: &str = "Show head n lines of CSV file. The result could be formatted 
as an aligned table by setting the --tabled flag.

Usage: 
  rsv.exe head [OPTIONS] <FILENAME>
  rsv head data.csv                   # print as the file is
  rsv head --tabled data.csv          # tabled 
  rsv head -t data.csv                # tabled too
  rsv head -s \\t -t data.csv          # CSV file with a tab separator

Arguments:
  <FILENAME>  File to open

Options:
  -s, --sep <SEP>  Separator [default: ,]
      --no-header  Whether the file has a header
  -n, --n <N>      Number of records to show [default: 20]
  -t, --tabled     print as a table
  -h, --help       Print help information (use `--help` for more detail)
";

pub const ESTIMATE_DESC: &str = "Fast estimate the number of lines. 
    
The command first read 20000 lines (except the header) from the CSV file, 
and then estimate average bytes of a line by dividing total bytes and
number of lines.

The total number of lines of CSV file is estimated according to file size
and average bytes per line.

The estimate is every fast.

Usage: 
  rsv.exe estimate <FILENAME>
  rsv.exe estimate data.csv

Arguments:
  <FILENAME>  File to open

Options:
  -h, --help  Print help information (use `--help` for more detail)
";

pub const CLEAN_DESC: &str = "Clean file with escape chars (e.g. \"). Other special strings
can also be cleaned.

Usage: 
  rsv.exe clean [OPTIONS] <FILENAME>
  rsv clean data.csv                                 # default to clean escape char \"
  rsv clean -e \"content-to-delete\" data.csv          # clean str to empty
  rsv clean -o new-file.csv data.csv                 # save to new-file.csv

Arguments:
  <FILENAME>  File to open

Options:
  -o, --output <F>       Output file, default to current-file-cleaned.csv
  -e, --escape <ESCAPE>  Escape char to clean [default: \"]
  -h, --help             Print help information";

pub const FLATTEN_DESC: &str =
    "Prints flattened records to view records one by one. Records are separated
by \"#\", which could be changed with the --delimiter flag. The command is 
similar to \"xsv flatten\" and \"\\G\" command in mysql.

Usage: 
  rsv.exe flatten [OPTIONS] <FILENAME>
  rsv flatten data.csv                       # default to show first 5 records
  rsv flatten -n 50 data.csv                 # show 50 records
  rsv flatten --delimiter \"--\" data.csv      # change delimiter to anything

Arguments:
  <FILENAME>  File to open

Options:
  -s, --sep <SEP>              Separator [default: ,]
      --no-header              Whether the file has a header
  -d, --delimiter <DELIMITER>  Line delimiter for printing [default: #]
  -n, --n <N>                  Number of records to show, n=-1 to show all [default: 5]
  -h, --help                   Print help information (use `--help` for more detail)
";

pub const SLICE_DESC: &str = "Prints a slice of rows from CSV file. 
        
The range is [start, end).    
If the start is omitted , the slice starts from the first record of CSV.
If the end is omitted, the slice continuous to the last record of CSV.
    
A slice length can be also specified by the --len flag,
when the length is specified, the end index is ignored.
    
A single data record can by retrieved by the --index (shortened as -i) flag.
when -i is specified, other flags (including start, end, length) are all ignored.

Usage: 
  rsv slice [Options] <FILENAME>
  rsv slice -s 100 -e 150 data.csv           # set start and end
  rsv slice -s 100 -l 50 data.csv            # set start and length
  rsv slice -s 100 -l 50 --export data.csv   # export to data-slice.csv
  rsv slice -e 10 --export data.csv          # set end and export data
  rsv slice -i 9 data.csv                    # the 10th line sliced only

Arguments:
  <FILENAME>  File to open

Options:
  -s, --start <START>    Start index of CSV [default: 0]
  -e, --end <END>        End index of CSV
  -l, --length <LENGTH>  Slice length
  -i, --index <INDEX>    Index for single record of CSV
      --no-header        Whether the file has a header
      --export           Export data to a current-file-slice.csv?
  -h, --help             Print help information (use `--help` for more detail)
";

pub const FREQUENCY_DESC: &str =
    "Frequency table for one column or multiple columns. The columns are specified
by the --cols or -c flag. Column syntax's are either -c 0,1,2,5 or -c 0-2,5.

The command is performant because row of CSV are analyzed in parallel
(based on Rayon library) after their read in. 

The frequency table is shown in descending order by default, but it can be 
changed with the --ascending or -a flag.

The frequency table is printed to stdout by default, but it can be saved 
to a data-frequency.csv new file by the --export flag.

Usage: 
  rsv.exe frequency [OPTIONS] <FILENAME>
  rsv frequency -c 0 data.csv              # default to the first column, descending order
  rsv frequency -c 0,1,2,5 data.csv        # columns 0, 1, 2, and 5
  rsv frequency -c 0-2,5 data.csv          # same as above
  rsv frequency -c 0-2 --export data.csv   # export result to data-frequency.csv
  rsv frequency -n 10 data.csv             # keep top 10 frequent items
  rsv frequency -a 10 data.csv             # in ascending order

Arguments:
  <FILENAME>  File to open

Options:
  -s, --sep <SEP>    Separator [default: ,]
      --no-header    Whether the file has a header
  -c, --cols <COLS>  Columns to generate frequency table [default: 0]
  -a, --ascending    Ascending order or not [default: false, meaning descending]
  -e, --export       Export result to a frequency.csv? [default: false]
  -n, --n <N>        Top N to keep in frequency table [default: 0, meaning all]
  -h, --help         Print help information

Column selection syntax:
-c 0,1,2,5   -->    cols [0,1,2,5]
-c 0-2,5     -->    same as cols [0,1,2,5]
";

pub const PARTITION_DESC: &str = "Partition (or split) a big and unordered file into separate files
based on a column value. Only one column can be specified at a time,
with the --col or -c flag. 

The output directory is created automatically within the current data
directory. Separate small files are named after unique column value. 

The command is performant because: (1) data is analyzed in chunks,
e.g., 256MB by default, other than by line by line, so that it avoid 
continuously opening and closing small files; (2) rows of CSV are 
analyzed in parallel (based on Rayon library) after their read in.

Usage: 
  rsv.exe partition [OPTIONS] <FILENAME>
  rsv partition data.csv             # default to first column and field separator of \",\" 
  rsv partition -c 1 data.csv        # second column to partition
  rsv partition -c 0 -s \\t data.csv  # first column, \\t separator

Arguments:
  <FILENAME>  File to open

Options:
  -s, --sep <SEP>  Separator [default: ,]
      --no-header  Whether the file has a header
  -c, --col <COL>  Columns to generate frequency table [default: 0]
  -h, --help       Print help information
";

pub const SELECT_DESC: &str = "Select rows and columns by filter. The row filter is either
0=a,b,c (meaning the first column has values of a, b, or c) or 0=a,b&1=c
(meaning the first column is a or b, AND the second column equals c).
Right now, only & (AND) operation is supported, | (OR) operation is not 
supported.

The columns to select are specified with the --cols or -c flag, supporting
syntax's like -c 0,1,2,5 or -c 0-2,5.

Output can be exported with the --export flag.

Usage: 
  rsv.exe select [OPTIONS] <FILENAME>
  rsv select -f 0=a,b,c data.csv             # first column has values of a, b, or c
  rsv select -f 0=a,b&1=c data.csv           # first column is a or b, AND the second column equals c
  rsv select -f 0=a,b&1=c --export data.csv  # export result
  rsv select -s \\t -f 0=a,b data.csv         # tab as separator

Arguments:
  <FILENAME>  File to open

Options:
  -s, --sep <SEP>        Separator [default: ,]
      --no-header        Whether the file has a header
  -c, --cols <COLS>      Columns to select, see column select syntax below; Default to select all columns
  -f, --filter <FILTER>  Row filter, support syntax 0=a,b,c or 0=a,b&1=c,d; Default to None
  -e, --export           Export results to a file named current-file-selected.csv?
  -h, --help             Print help information

Column selection syntax:
-c 0,1,2,5   -->    cols [0,1,2,5]
-c 0-2,5     -->    same as cols [0,1,2,5]
";

pub const STATS_DESC: &str = "Statistics for every column, including min, max, mean, unique, null.
Within the command, every column is regarded as either an Int, Float or String.
When the column is String, min, max, mean are ignored. When the column is Float,
the unique stat is ignored.

The command process data in parallel, so that it is fast.

Usage: 
  rsv stats [OPTIONS] <FILENAME>
  rsv stats data.csv                       # all columns
  rsv stats -c 0,1 data.csv                # first two columns
  rsv stats -c 0,1 --export data.csv       # export statistics to data-stats.csv

Arguments:
  <FILENAME>  File to open

Options:
  -s, --sep <SEP>    Separator [default: ,]
      --no-header    Whether the file has a header
  -c, --cols <COLS>  Columns to select, support syntax 0,1,3 or 0-4, including 4; Default to select all columns
  -e, --export       Export results to a file named current-file-selected.csv?
  -h, --help         Print help information
";