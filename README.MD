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