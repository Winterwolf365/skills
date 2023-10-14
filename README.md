# skills
practice for an upcomming competition, by reading data from an txt file

## instalation
for windows you can download the executable file along the test data file in the releases of this repository.
And for other platforms make sure you have [rust](https://rustup.rs/) and [git](https://git-scm.com/downloads/) installed
than clone this repository and change directory to the cloned repository and build it using cargo for release
```bash
git clone https://github.com/Winterwolf365/skills.git
cd skills
cargo run --release
```
than the executable file will be located in /target/release/{filename} and you can find the test data file in the releases of this repository.

## Usage
```
Usage: skills.exe [OPTIONS] --file <FILE>

Options:
  -f, --file <FILE>          Location of data file
  -s, --sort <SORT>          Sort the output [default: id] [possible values: id, publication-date, author, genre, title, total-pages, rating, language]
  -o, --output <OUTPUT>      Format the output [default: rust-struct] [possible values: rust-struct, json, yaml]
  -i, --id <ID>              Search for an specific id
  -a, --author <AUTHOR>      Search for an author
  -g, --genre <GENRE>        Search for an genre
  -t, --title <TITLE>        Search for an title
  -l, --language <LANGUAGE>  Search for an language
  -h, --help                 Print help
  -V, --version              Print version
```
