BreakoutLang is a "simmeringly slow"™ haskell-like (semi) lazily evaluated language.

I am using it as an avenue to learn more about the rust type system, functional programming, concurrency, and multithreading.

I also plan to put features in it that I think are interesting/funny/cool from other languages. Some examples/plans include:

- The Racket "cond" being the only branching logic (because Racket brings back good memories)
- Filepaths as primitives from Nix (because this is a great idea even for a non DSL)
- Reverse Polish Notation for everything (including primitive operations) because why not and also ♥ Racket 

Planned features:

- Somehow supporting direct access to the rust standard library
- Trivial support for concurrency and multithreading (just a keyword to put a function in its own thread/greenthread)
- "Staged" functions??? - idrk what this entails yet
- Hopefully above average error messages 
- a REPL!?

Here is some example code:

```
int func fib := a : int => cond (< a 2) 1 (+ fib (- a 1) fib (- a 2))

int main := fib 17 
```

Installation:

1. Install rust
2. Clone the repo and run `cargo build --release`
3. Run `./target/release/breakout <your code's file path>`
