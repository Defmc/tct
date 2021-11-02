# Tiny Commands Toolchain: TCT
A **1.68MB**(release profile) monolithic commands kit for casual terminal usage without any run-time dependency.

## Usage
For execute a command, just run the `tct` with the first argument being the same **all commands require at least 1 argument**.

Examples:
| command |  launch string               |
| `cat`   | `tct cat some_file`          |
| `grep`  | `tct cat a_string some_file` |
| `mkdir` | `tct mkdir new_dir`          |

## Benchmarks
Comparision of some tools. Everything with `gnu` prefix means that's from `gnu-coreutils`'s Arch package (version `9.0-2`).

**Notes**: Some tools as `rg` and `bat` has a **LOT** of features and is beautiful for interactive usage. By this, some benchmarks will show them as slower than other commands.

### `cat`
On a 10KB file:
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat lorem.txt` | 1.3 ± 1.9 | 0.0 | 9.4 | 1.00 |
| `./target/release/tct cat lorem.txt` | 2.0 ± 2.5 | 0.0 | 14.3 | 1.56 ± 3.07 |
| `bat lorem.txt` | 131.3 ± 7.8 | 118.3 | 145.0 | 104.31 ± 157.15 |

On a 10MB file:
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat biglorem.txt` | 10.0 ± 5.5 | 1.1 | 34.8 | 4.85 ± 6.69 |
| `./target/release/tct cat biglorem.txt` | 2.1 ± 2.6 | 0.0 | 13.7 | 1.00 |
| `bat biglorem.txt` | 157.4 ± 6.6 | 145.4 | 169.3 | 76.78 ± 96.95 |

### grep
On a 10KB file searching by all `Ipsum` string ocurrences:
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `grep Ipsum lorem.txt` | 6.3 ± 2.8 | 1.7 | 11.4 | 2.55 ± 2.18 |
| `./target/release/tct grep Ipsum lorem.txt` | 2.4 ± 1.8 | 0.0 | 6.0 | 1.00 |
| `rg Ipsum lorem.txt` | 7.7 ± 3.1 | 3.7 | 14.1 | 3.15 ± 2.63 |

On a 10MB file searching by all `Ipsum` string ocurrences:
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `grep Ipsum biglorem.txt` | 5.4 ± 2.8 | 1.7 | 15.8 | 1.00 |
| `./target/release/tct grep Ipsum biglorem.txt` | 41.7 ± 6.6 | 36.5 | 63.2 | 7.72 ± 4.24 |
| `rg Ipsum biglorem.txt` | 23.0 ± 5.1 | 13.9 | 33.0 | 4.26 ± 2.43 |

## Compile Tips
Be sure to compile with `release` profile (`cargo build --release`). If you're compiling for local usage, sill can set `RUSTFLAGS` with `-C target-cpu=native` before compile (`RUSTFLAGS='-C target-cpu=native' cargo build --release` on \*NIX, `set RUSTFLAGS='-C target-cpu=native' ; cargo build --release` on Windows) to use your CPU specific instructions for local optimization.

## TODO
- [] (Add more benchmarks)
- [] (Optimize `grep` for large files)
- [] (Improve README)
- [] (Add `ls` command)
- [] (Update manpages)
