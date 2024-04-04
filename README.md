# evenpw

A password generator that aims to generate easy to type passwords.

> **The Problem**
> Often when you type a password you have awkward finger positions and use both hands unevenly.
> This generator aims to reduce this.

## Installation

Using Cargo:

    cargo install --git https://github.com/lukas412/evenpw.git

## Usage

Generate a password with 16 chars:

    evenpw

Generate a password with custom length:

    # Please keep in mind, that this password generator can make the passwords more predictable.
    # Therefore the password should be longer.

    evenpw -l 20
    evenpw --length 20

Generate many passwords:

    evenpw -c 10
    evenpw --count 10
