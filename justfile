set dotenv-load

day := `date +%d`
dayWithout0 := trim_start_match(day, "0")
year := "2024"
file := "src/bin/" + day + ".rs"

# Format, lint, and run the program for today.
run:
    rustfmt {{file}}
    cargo clippy
    # Hide warning here because we just ran clippy
    RUSTFLAGS=-Awarnings cargo build --release --bin {{day}}
    time ./target/release/{{day}}

# Begin working on todays problem.
# Downloads input, creates template and opens the problem and code.
begin: _folders
    cat template.rs | sed -e "s/DAY/{{day}}/g" >> src/bin/{{day}}.rs
    rustfmt {{file}}
    curl --silent "https://adventofcode.com/{{year}}/day/{{dayWithout0}}/input" -H "Cookie: session=$AOC_SESSION" > "input/{{day}}"
    touch input/{{day}}-test
    $EDITOR {{file}}
    $EDITOR input/{{day}}-test
    open "https://adventofcode.com/{{year}}/day/{{dayWithout0}}"

# Makes sure that folders exist
_folders:
    mkdir -p input src/bin

# Creates all input files. Use this to start using the repo.
init: _folders
    #!/usr/bin/env bash
    for i in $(seq -w 1 25);
    do
        touch input/$i
        touch input/$i-test
    done
