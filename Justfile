set dotenv-load

#day := `date +%d`
day := `if [ "$(date +%Y%m%d)" -gt "20241225" ]; then echo 25; else date +%d; fi`

alias r := run_today
alias t := test_today
alias p1 := part1
alias p2 := part2
alias c := clean

_default: test_today

#clean inputs and cargo files
clean:
	rm -r inputs
	cargo clean

#benchmak everything
bench:
	cargo bench

#benchmak only the <days>
bench_days +days:
	cargo bench {{days}}

#run all days in release mode
[group('run')]
run_all: 
	cargo run -q --release -- $(seq {{day}})

#test all days
[group('test')]
test_all:
	cargo test

#run today in release mode
[group('run')]
[group('useful')]
run_today:
	cargo run -q --release -- {{day}}

#run tests for today 
[group('test')]
[group('useful')]
test_today:
	cargo test day{{day}}

#run the test for part 1 of today
[group('test')]
part1:
	cargo test day{{day}}::tests::part1

#run the test for part 2 of today
[group('test')]
part2:
	cargo test day{{day}}::tests::part2

#run <days> in release mode
[group('run')]
days +days:
	cargo run -q --release -- {{days}}

#run tests for <days>
[group('test')]
tests +days:
	#!/bin/bash
	for day in {{days}};
	do
		formatted_day=$(printf "%02d" $day)
		cargo test day$formatted_day
	done
