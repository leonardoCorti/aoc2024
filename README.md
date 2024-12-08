# advent of code 2024
these are my [advent of code](https://adventofcode.com/2024) solutions for 2024 in Rust. To use this you will need to either have curl in the Path or create manually a inputs folder with your inputs formatted like 1.txt, 2.txt etc... 

there is a Justfile with some commands to make it easier to use, it needs bash
```just
Available recipes:
    bench            # benchmak everything
    bench_days +days # benchmak only the <days>
    clean            # clean inputs and cargo files
    c                # alias for `clean`

    [run]
    days +days       # run <days> in release mode
    run_all          # run all days in release mode
    run_today        # run today in release mode
    r                # alias for `run_today`

    [test]
    part1            # run the test for part 1 of today
    p1               # alias for `part1`
    part2            # run the test for part 2 of today
    p2               # alias for `part2`
    test_all         # test all days
    test_today       # run tests for today
    t                # alias for `test_today`
    tests +days      # run tests for <days>

    [useful]
    run_today        # run today in release mode
    r                # alias for `run_today`
    test_today       # run tests for today
    t                # alias for `test_today`
```

## performance

| day | part | time mean |
| --- | ---- | --------- |
| 1   | 1    | 86.809 µs |
| 1   | 2    | 123.12 µs |
| 2   | 1    | 169.50 µs |
| 2   | 2    | 2.3958 ms |
| 3   | 1    | 407.92 µs |
| 3   | 2    | 746.34 µs |
| 4   | 1    | 2.0572 ms |
| 4   | 2    | 538.89 µs |
| 5   | 1    | 2.7889 ms |
| 5   | 2    | 1.1875 ms |
| 6   | 1    | 74.101 µs |
| 6   | 2    | 43.495 ms |
| 7   | 1    | 18.465 ms |
| 7   | 2    | 565.95 ms |
| 8   | 1    | 23.774 µs |
| 8   | 2    | 60.094 µs |

