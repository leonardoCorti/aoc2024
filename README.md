# advent of code 2024
these are my [advent of code](https://adventofcode.com/2024) solutions for 2024 in Rust. To use this you will need to either have curl in the Path or create manually a inputs folder with your inputs formatted like 1.txt, 2.txt etc... 

there is a Justfile with some commands to make it easier to use, it needs bash
```just
Available recipes:
    clean       # clean inputs and cargo files
    c           # alias for `clean`

    [run]
    days +days  # run <days> in release mode
    run_all     # run all days in release mode
    run_today   # run today in release mode
    r           # alias for `run_today`

    [test]
    part1       # run the test for part 1 of today
    p1          # alias for `part1`
    part2       # run the test for part 2 of today
    p2          # alias for `part2`
    test_all    # test all days
    test_today  # run tests for today
    t           # alias for `test_today`
    tests +days # run tests for <days>

    [useful]
    run_today   # run today in release mode
    r           # alias for `run_today`
    test_today  # run tests for today
    t           # alias for `test_today`
```
