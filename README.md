# evenpw

A password generator that aims to generate easy to type passwords.

Often when you type a password you have awkward finger positions
and use both hands unevenly.\
This generator aims to reduce this.

It takes the current hand into account
and places the next key press on the other hand.

When the other hand is pressing shift,
you hit two keys in a row with the same hand.

Currently only the QWERTZ layout is implemented.

## Installation

Using Cargo:

    cargo install --git https://github.com/lukas412/evenpw.git

## Usage

Generate a password with 16 chars:

    evenpw

Generate a password with custom length:

    # Please keep in mind, that this password generator can make the passwords better predictable.
    # Therefore the password should be longer.

    evenpw -l 20
    evenpw --length 20

Generate many passwords:

    evenpw -n 10
    evenpw --count 10

Copy last generated password to clipboard:

    evenpw -c
    evenpw --copy

