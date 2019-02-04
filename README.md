# Small propositional logic parser

If you are unable to install a Rust compiler or prefer not to, you can just click the link below and test it out. You can change the formula on line 338.

https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=a718bb638a434f93f1551f4789bbccc9

## Build:

Install rustc and/or cargo with rustup if it's not installed already.

With Cargo:

`$ cargo build` tai `$ cargo build --release`

With just rustc (inside the src folder):

`$ rustc main.rs`

## Usage:

Bash (rustc): `./main <lauseke/formula>`  
Cmd (rustc): `main <lauseke/formula>`  
Cargo: `cargo run <lauseke/formula>`

**Example 1:**
```
$ cargo run '(A v B)'
tokenized: [ParOpen, Var('A'), Or, Var('B'), ParClose]
tree: ( A ) | ( B )
Unknown variable: B
Unknown variable: A
tvars: [('B', false), ('A', false)]
Is not a tautology
```

**Example 2:**
```
$ cargo run '((!A => B) ^ (!A => !B)) => A'
tokenized: [...]
tree: [...]
Unknown variable: A
Unknown variable: B
tvars: [('A', false), ('B', false)]
tvars: [('A', true), ('B', false)]
tvars: [('A', false), ('B', true)]
tvars: [('A', true), ('B', true)]
Is a tautology
```
