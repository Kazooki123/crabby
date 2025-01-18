# Crabby: The Modern Programming Language Written in Rust
<!-- ALL-CONTRIBUTORS-BADGE:START - Do not remove or modify this section -->
[![All Contributors](https://img.shields.io/badge/all_contributors-2-orange.svg?style=flat-square)](#contributors-)
<!-- ALL-CONTRIBUTORS-BADGE:END -->

![StarloSearch](https://avatars.githubusercontent.com/u/139462470?s=48&v=4)

![GitHub License](https://img.shields.io/github/license/Kazooki123/crabby?style=for-the-badge&logo=gnu&logoColor=%23A42E2B)

## Introduction

![Logo](https://github.com/Kazooki123/crabby/blob/main/crabbylogo.jpg)

Crabby is a modern High-level, multi-paradigm, functional programming language.
With a **Pythonic** style syntax, it is designed to be readable, ease-to-use for beginners, and powerful enough for advanced users.

## What Change?

As you may notice, Crabby has been through phases of rewrite, and if you're thinking that
**Crabby** is abounded, well... You're Wrong!

Crabby has been officially and well be written in **Rust** due to it's memory safety and type checkings compared to *C*.

Here's what changed:

1. It has Pythonic-style of Syntax BUT with a functional approach
2. It's purely functional (not yet for now)
3. JIT compile time
4. Runtime & Type checking on the work.
5. Error handling on the work
6. More parsing and features!

## Installation

1. Git clone:

```bash
git clone https://github.com/Kazooki123/crabby.git
```

2. Make sure to have `rust` and `cargo` installed

```bash
cargo --version
```

3. Build it and Test it for yourself

```bash
cargo build
cargo run examples/example.crab
```

## Syntax

It's default file format is a `.crab` or `.cb`
But for now it's `.crab`

example.crab:

```js
let x = 42
let y = 314
let message = "Hello, Crabby!"

print(x)
print(y)
print(message)
```

helloworld.crab:

```js
print("Hello, World!")
```

math.crab:

```js
// addition
let x1 = 1
let y1 = 2

// multiplication
let x2 = 4
let y2 = 7

// subtraction
let x3 = 10
let y3 = 3

// division
let x4 = 10
let y4 = 3

print(x1 + y1)
print(x2 * y2)
print(x3 - y3)
print(x4 / y4)
```

ifelse.crab:

```js
// if-else statements

let true = 1

if true: {
    print("True!")
} else {
    print("Nope!")
}

```

Note: **Crabby** supports commenting, use `//` to comment out a code or leave a silly ahh message :3

## Package Manager when?

It's in development and I'm still planning how to use in it.
**Crabby** doesn't support things like `import` or `export` for now
but well do in the future 😄

## Contributing

Crabby is open to contributions! Feel free to open an issue or a pull request.
Make sure to read the [contributing guidelines](CONTRIBUTING.md) before getting started.

## LICENSE

Crabby is licensed under the GNU General Public License v3.0.

## Old Contributors ✨

Thanks goes to these wonderful people that used to help this project! 👨‍💻💻:

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/Satvik-2727"><img src="https://avatars.githubusercontent.com/u/87568817?v=4?s=100" width="100px;" alt="Mr.Coder"/><br /><sub><b>Mr.Coder</b></sub></a><br /><a href="https://github.com/Kazooki123/crabby/commits?author=Satvik-2727" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/Scarleyegaming"><img src="https://avatars.githubusercontent.com/u/93965392?v=4?s=100" width="100px;" alt="Saturo"/><br /><sub><b>Saturo</b></sub></a><br /><a href="https://github.com/Kazooki123/crabby/commits?author=Scarleyegaming" title="Code">💻</a></td>
      <td align="center" valign="top" width="14.28%"><a href="https://tiramify.dev"><img src="https://avatars.githubusercontent.com/u/94789999?v=4?s=100" width="100px;" alt="Trnx"/><br /><sub><b>Trnx</b></sub></a><br /><a href="https://github.com/Kazooki123/crabby/commits?author=trnxdev" title="Code">💻</a></td>
    </tr>
  </tbody>
</table>
