# Faster Diagnostic Serializer (for Rust Analyzer)

This is meant as a proof of concept for a faster diagnostic serializer for Rust Analyzer. The current diagnostic serializer is slow because it depends on the serde and format subsystems.

## Changes

Here are the changes made to improve performance:

### Hard-code the static part of the diagnostic

The static part of the diagnostic is hard-coded (key names and commas, braces etc).

### Use itoa instead of the format subsystem for numbers

`itoa` is a crate that converts integers to strings without using the format subsystem.

### Use an improved formatting function

We use a similar function to the one provided by `ufmt`.

## Running the benchmark

To run the benchmark, run the following command:

```bash
cargo bench
```

## Results

The benchmark results are as follows:

```
serde_serialize_diagnostic/
                        time:   [2.5596 µs 2.5817 µs 2.6084 µs]
                        change: [+1.0386% +3.0704% +5.2016%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 10 outliers among 100 measurements (10.00%)
  7 (7.00%) high mild
  3 (3.00%) high severe

sonic_serialize_diagnostic/
                        time:   [1.7873 µs 1.8272 µs 1.8750 µs]
                        change: [+9.9039% +13.142% +18.029%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 9 outliers among 100 measurements (9.00%)
  3 (3.00%) high mild
  6 (6.00%) high severe

fast_serialize_error_diagnostic/
                        time:   [793.19 ns 794.61 ns 796.09 ns]
                        change: [-0.0517% +0.8713% +1.4902%] (p = 0.02 < 0.05)
                        Change within noise threshold.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe
```

## Contributing

If you have any ideas on how to improve the performance of the diagnostic serializer, feel free to open an issue or a pull request. After you've made a change, make sure to run the benchmark to see if the performance has improved and run the test suite to make sure that everything still works as expected.

To run the test suite, run the following command:

```bash
cargo test
```