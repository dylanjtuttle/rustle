# rustle
Welcome to Rustle, a command-line based version of Wordle, written in Rust!

Enter your guesses one by one, and the game will give you information about the secret word.

![First guess is crane](https://ibb.co/f2GC6pW)

![Second guess is pious](https://ibb.co/MNrsG9Y)

If a letter is green, that letter appears at that position in the secret word. If a letter is yellow, that letter appears in the secret word, but not at that position.

![Game over, you win](https://ibb.co/pZBpG94)

You get six (6) tries to correctly guess the word, and if you do, you win!

## Installation
- Ensure you have installed Rust on your device. [Follow these instructions to do so](https://www.rust-lang.org/tools/install)
- Clone this repository with the following command:
```bash
git clone https://github.com/dylanjtuttle/rustle
```
- Enter into the directory created:
```bash
cd rustle
```
- And finally, run the game!
```bash
cargo run
```