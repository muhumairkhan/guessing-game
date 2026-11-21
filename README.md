# Guess It! 🎲

A classic terminal-based number guessing game built in Rust. The program generates a secret random number between 1 and 100 and challenges you to guess it with interactive feedback and ASCII art banners.

## Features

- **Random Number Generation:** Picks a fresh secret number (`1..=100`) every round using the `rand` crate.
- **Interactive Hints:** Tells you whether your guess is too low (`less`) or too high (`greater`).
- **Continuous Play:** Automatically starts a new round once you win, allowing endless replayability.
- **ASCII Art Banners:** Custom welcome and victory screens right in your terminal.
- **Graceful Input Handling:** Uses `Box<dyn Error>` to cleanly handle non-numeric inputs or I/O errors by resetting the round.

## Prerequisites

- [Rust](https://rust-lang.org) (Edition 2021 or 2024 recommended)
- Cargo (included with Rust)

## Dependencies

This project uses `rand` (v0.9+). Make sure your `Cargo.toml` includes:

```toml
[dependencies]
rand = "0.9"
```

## Getting Started

1. **Clone the repository** (or create a new Cargo project and paste your code into `src/main.rs`):

   ```bash
   git clone https://github.com
   cd guess-it
   ```

2. **Build and run the game:**
   ```bash
   cargo run
   ```

## How to Play

1. Launch the game via `cargo run`.
2. Type a number between **1** and **100** and press `Enter`.
3. Follow the prompts (`Your guess is less` or `Your guess is greater`) to narrow down the secret number.
4. Guess the exact number to trigger the **YOU WIN!** banner and start a new round.
5. Press `Ctrl + C` at any time to exit the game.

## Example Gameplay

```text
==================================================
   ____ _   _ _____ ____ ____    ___ _____ _
  / ___| | | | ____/ ___/ ___|  |_ _|_   _| |

 | |  _| | | |  _| \___ \___ \   | |  | | | |
 | |_| | |_| | |___ ___) |__) |  | |  | | |_|
  \____|\___/|_____|____/____/  |___| |_| (_)

==================================================
  Welcome to the Number Guessing Game!
  I've picked a secret number between 1 and 100.
  Can you guess what it is?
==================================================

Enter a number to guess:
50
Your guess is less. Try again.
Enter a number to guess:
75
Your guess is greater. Try again.
Enter a number to guess:
63

__   __ ___   _   _    __        __ ___  _   _  _
\ \ / // _ \ | | | |   \ \      / /|_ _|| \ | || |
 \ V /| | | || | | |    \ \ /\ / /  | | |  \| || |

  | | | |_| || |_| |     \ V  V /   | | | |\  ||_|
  |_|  \___/  \___/       \_/\_/   |___||_| \_|(_)
```

## License

This project is open source and available under the [MIT License](LICENSE).
