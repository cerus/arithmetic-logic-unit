# arithmetic-logic-unit
ALU representation in Rust

## Disclaimer
This is my very first serious Rust project. I really appreciate constructive criticism so please let me know if there's something that I could do better.

## Contents of this repository
This repository contains
- a [binary representation](src/model/binary_representation.rs) capable of converting decimals to binary and back
- logic gates:
    - [and gate](src/model/gates/and_gate.rs)
    - [or gate](src/model/gates/or_gate.rs)
    - [xor gate](src/model/gates/xor_gate.rs)
    - [nand gate](src/model/gates/nand_gate.rs)
    - [not gate](src/model/gates/not_gate.rs)
- adders:
    - [half adder](src/model/adder/half_adder.rs)
    - [full adder](src/model/adder/full_adder.rs)
    - [ripple carry adder](src/model/adder/ripple_carry_adder.rs)
- subtractors:
    - [half subtractor](src/model/subtractor/half_subtractor.rs)
    - [full subtractor](src/model/subtractor/full_subtractor.rs)
    - [ripple carry subtractor](src/model/subtractor/ripple_carry_subtractor.rs)

## Feedback
Please open an issue for feedback. You can also contact me on Discord (Cerus#5149).