set dotenv-load

#day := `date +%d`
day := `if [ "$(date +%Y%m%d)" -gt "20241225" ]; then echo 25; else date +%d; fi`

alias r := run_today
alias t := test_today
alias p1 := part1
alias p2 := part2

_default: test_today

#run all days in release mode
run_all: 
	cargo run -q --release -- $(seq {{day}})

#test all days
test_all:
	cargo test -- $(seq {{day}})

#run today in release mode
run_today:
	cargo run -q --release -- {{day}}

#run tests for today 
test_today:
	cargo test day{{day}}

#run the test for part 1 of today
part1:
	cargo test day{{day}}::tests::part1

#run the test for part 2 of today
part2:
	cargo test day{{day}}::tests::part2

#run <days> in release mode
days +days:
	cargo run -q --release -- {{days}}

