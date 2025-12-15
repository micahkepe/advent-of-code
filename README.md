# Advent of Code

My solutions to the [Advent of Code](https://adventofcode.com/) challenges.

> [!NOTE]
> The original challenge inputs are not included in this repository as per the
> [Advent of Code rules](https://adventofcode.com/2025/about#faq_copying):
>
> > **Can I copy/redistribute part of Advent of Code?** Please don't. Advent of
> > Code is free to use, not free to copy. If you're posting a code repository
> > somewhere, please don't include parts of Advent of Code like the puzzle text
> > or your inputs. If you're making a website, please don't make it look like
> > Advent of Code or name it something similar.

## Setup

1. **Set up your session cookie**

   Copy the example environment file:

   ```bash
   cp .env.example .env
   ```

   Get your session cookie from [adventofcode.com](https://adventofcode.com):
   - Log in to Advent of Code
   - Open DevTools (F12)
   - Go to **Application** &rarr; **Cookies** &rarr; `https://adventofcode.com`
   - Copy the value of the `session` cookie
   - Paste it into `.env`:

   ```bash
   AOC_SESSION_COOKIE=your_session_cookie_here
   ```

2. **Create a new day**

   ```bash
   # From project root - creates for current year
   cargo xtask gen --day 1

   # From within a year directory (e.g., cd 2025)
   cargo xtask gen --day 1

   # Specify a year explicitly
   cargo xtask gen --day 1 --year 2024
   ```

   This will:
   - Create `src/bin/day_XX.rs` from the template
   - Fetch and save the puzzle input to `data/day-XX-input.txt`

3. **Run your solution**

   ```bash
   cd <year>
   cargo run --bin day_01
   ```

4. **Run tests**

   ```bash
   cargo test --bin day_01
   ```

## [`cargo-xtask`](https://github.com/matklad/cargo-xtask) Automation

```
Automation to create a new day for Advent of Code

Usage: xtask <COMMAND>

Commands:
  gen   Generate Rust template file and retrieve puzzle input for specified day and year
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

<details>
<summary><code>gen</code> Subcommand</summary>

```
Automation to create a new day for Advent of Code

Usage: xtask <COMMAND>

Commands:
gen Generate Rust template file and retrieve puzzle input for specified day and year
help Print this message or the help of the given subcommand(s)

Options:
-h, --help Print help
```

</details>

### Examples

```bash
# Create day 5 for current year
cargo xtask gen --day 5

# Create day 10 for 2024
cargo xtask gen --day 10 --year 2024

# Short form
cargo xtask gen -d 3 -y 2023
```

## License

Licensed under MIT license, see [LICENSE](LICENSE).

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Acknowledgements

[Eric Wastl](https://was.tl/) for the AoC project!
