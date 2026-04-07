This is a libray that helps the users convert transaction data between different formats, namely: csv, binary and txt. It also has two binary targets, described below.

### ypbank_compare

```
cargo run --bin ypbank_compare -- \
  --file1 <path_to_file> \
  --format1 <format> \
  --file2 <path_to_file> \
  --format2 <format>

# where
<format> :=
| csv
| binary
| txt
```

### ypbank_converter

```
cargo run --bin ypbank_converter -- \
  --input <input_file> \
  --input-format <format> \
  --output-format <format> \
  > output_file.txt

# where
<format> :=
| csv
| binary
| txt
```
