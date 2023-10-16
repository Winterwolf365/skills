# **Skills**
Practice for an upcoming competition, by reading data from a TXT file.

## Installation
For windows you can download the executable file along the test data file in the [releases](https://github.com/Winterwolf365/skills/releases) of this repository.
And for other platforms make sure you have [rust](https://rustup.rs) and [git](https://git-scm.com/downloads) installed, 
then clone this repository and change directory to the cloned repository and build it using cargo for release.
```
git clone https://github.com/Winterwolf365/skills.git
cd skills
cargo build --release
```
Then the executable file will be located in /target/release/{filename}, and you can find the test data file in the [releases](https://github.com/Winterwolf365/skills/releases) of this repository.

Tip: add the executable file to your path, so you can use it anywhere in your terminal!

## Usage
```
Usage: skills.exe [OPTIONS] --read <READ>

Options:
  -r, --read <READ>          Read the data from a file location
  -s, --sort <SORT>          Sort the output [default: id] [possible values: id, publication-date, author, genre, title, total-pages, rating, language]
  -f, --format <FORMAT>      Format the output [default: debug] [possible values: debug, json, yaml]
  -w, --write <WRITE>        Write the output to a file location
  -i, --id <ID>              Search for an specific id
  -a, --author <AUTHOR>      Search for an author
  -g, --genre <GENRE>        Search for an genre
  -t, --title <TITLE>        Search for an title
  -l, --language <LANGUAGE>  Search for an language
  -h, --help                 Print help
  -V, --version              Print version
```
