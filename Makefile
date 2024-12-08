day ?= "1"
file ?= day$(shell printf "%02d" $(day))

MAIN_RS = rust/src/main.rs
MAIN_GO = go/main.go

init:
	@wget --load-cookies=cookies.txt -O ./inputs/$(file).txt https://adventofcode.com/2024/day/$(day)/input 
	@touch rust/src/$(file).rs
	@touch go/$(file)/$(file).go
	@grep -q "pub mod $(file);" $(MAIN_RS) || make update_main
	@grep -q "aoc24/$(file)" $(MAIN_GO) || make update_go

update_rs:
	@sed -i "s|/\* insert-run \*/|$(day) => println!(\"{:?}\", $(file)::run(\&input)),\\n\\t\\t/* insert-run */|g" $(MAIN_RS)
	@sed -i "s|/\* insert-pub-mod \*/|pub mod $(file);\\n/* insert-pub-mod */|g" $(MAIN_RS)

update_go:
	@sed -i 's|/\* insert-import \*/|"aoc24/$(file)"\n\t/\* insert-import \*/|g' $(MAIN_GO)
	@sed -i 's|/\* insert-case \*/|case $(day):\n\t\tfmt.Println($(file).Run(input))\n\t/\* insert-case \*/|g' $(MAIN_GO)
	@goimports -w go/main.go

run-rs:
	@cd rust && cargo run -- $(day)

run-go:
	@cd go && go run . $(day)