set dotenv-load

day := `date +%d`
dayWithout0 := trim_start_match(day, "0")
year := "2023"
file := "src/bin/" + day + ".rs"

# Format, lint, and run the program for today.
run:
    cargo clippy
    # Hide warning here because we just ran clippy
    RUSTFLAGS=-Awarnings cargo build --release --bin {{day}}
    time ./target/release/{{day}}

# Begin working on todays problem. Downloads input, creates template and opens the problem and code.
begin: _input-folder
    echo "const INPUT: &str = include_str!(\"../../input/{{day}}\"); const TEST_INPUT: &str = include_str!(\"../../input/{{day}}-test\"); fn main() {}" >> {{file}}
    rustfmt {{file}}
    curl --silent "https://adventofcode.com/{{year}}/day/{{dayWithout0}}/input" -H "Cookie: session=$AOC_SESSION" > "input/{{day}}"
    touch input/{{day}}-test
    rustrover {{file}}
    rustrover input/{{day}}-test
    open "https://adventofcode.com/{{year}}/day/{{dayWithout0}}"

_input-folder:
    mkdir -p input
