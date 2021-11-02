# Tiny Commands Toolchain: TCT
A 1.62MB (release profile) monolithic commands kit for casual terminal usage without any run-time dependency. MMade in 171 LOC (lines of code).

## Usage
For execute a command, just run the `tct` with the first argument being the same **all commands require at least 1 argument**.

Examples:
| command | launch string |
| :--- | ---: |
| `cat` | `tct cat some_file` |
| `grep` | `tct cat a_string some_file` |
| `mkdir` | `tct mkdir new_dir` |

## Benchmarks
Comparision of some tools. Everything with `gnu` prefix means that's from `gnu-coreutils`'s Arch package (version `9.0-2`).

**Notes**: Some tools as `rg` and `bat` has a **LOT** of features and is beautiful for interactive usage. By this, some benchmarks will show them as slower than other commands.

All benchmarks are made on a 1.89GB RAM 2.6GHz Celeron Dual Core CPU machine~/potato~.

### `cat`
On a 10KB file:
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat lorem.txt` | 1.7 ± 1.4 | 0.0 | 5.2 | 1.00 |
| `./target/release/tct cat lorem.txt` | 2.3 ± 1.7 | 0.0 | 5.9 | 1.33 ± 1.45 |
| `bat lorem.txt` | 97.8 ± 5.9 | 92.6 | 119.5 | 56.09 ± 45.92 |

On a 10MB file:
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `cat biglorem.txt` | 5.9 ± 2.7 | 1.7 | 12.8 | 2.65 ± 2.36 |
| `./target/release/tct cat biglorem.txt` | 2.2 ± 1.7 | 0.0 | 6.4 | 1.00 |
| `bat biglorem.txt` | 121.7 ± 6.4 | 116.4 | 139.6 | 54.39 ± 41.59 |

### `grep`
On a 10KB file searching by all `Ipsum` string ocurrences:
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `grep Ipsum lorem.txt` | 5.9 ± 2.8 | 1.5 | 10.9 | 2.56 ± 2.32 |
| `./target/release/tct grep Ipsum lorem.txt` | 2.3 ± 1.8 | 0.0 | 5.9 | 1.00 |
| `rg Ipsum lorem.txt` | 8.5 ± 2.9 | 3.5 | 14.2 | 3.71 ± 3.13 |

On a 10MB file searching by all `Ipsum` string ocurrences:
| Command | Mean [ms] | Min [ms] | Max [ms] | Relative |
|:---|---:|---:|---:|---:|
| `grep Ipsum biglorem.txt` | 5.7 ± 2.9 | 1.7 | 11.6 | 1.00 |
| `./target/release/tct grep Ipsum biglorem.txt` | 36.3 ± 5.7 | 32.3 | 60.4 | 6.33 ± 3.30 |
| `rg Ipsum biglorem.txt` | 24.8 ± 5.1 | 13.8 | 33.1 | 4.33 ± 2.33 |

## Compile Tips
Be sure to compile with `release` profile (`cargo build --release`). If you're compiling for local usage, sill can set `RUSTFLAGS` with `-C target-cpu=native` before compile (`RUSTFLAGS='-C target-cpu=native' cargo build --release` on \*NIX, `set RUSTFLAGS='-C target-cpu=native' ; cargo build --release` on Windows) to use your CPU specific instructions for local optimization.

## TODO
- [ ] (Add more benchmarks)
- [ ] (Optimize `grep` for large files)
- [ ] (Improve README)
- [ ] (Add `ls` command)
- [ ] (Update manpages)
