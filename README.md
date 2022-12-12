# CLI Calculator in Rust lang
This project is a very simple calculator in CLI made in Rust, that uses trees for operate its expressions.

For while, the code is separated in two important modules: [term](#term) and [calculator](#operation).

## Term
Here is the TermNode enum, which is a node that composes our operation tree.

A TermNode instace can be a Number or a Operation, which carry a value of type f32 and a operation of type [Operation](#operation), respectively.

## Operation
Here is the Operation enum, which can be (for while) a Addition or a Subtract.

The function operate, take the the fields passed in construction and operate it accordingly to his kind.

These fields must be TermNodes of any kind.
