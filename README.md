## Overview
This Rust program explains the implementation of the Pedersen hash function, which is extensively used in cryptographic protocols. It employs modular exponentiation to hash a vector of values given a set of generators and a modulus.
## Functions Implemented:
- `mod_exp:` This function calculates the modular exponentiation. It computes the result of `base^exp % modulus` using an efficient algorithm called **exponentiation by squaring**.
- `pedersen_hash:` This function implements the **Pedersen hash**. It takes as input a list of generators, a list of values, and a modulus. It computes the hash by performing the modular exponentiation of each generator raised to its corresponding value, then multiplies all the results together modulo the given modulus.
## Example
For the given modulus $p=13$, generators $g=[2,6,7]$, and values $x=[3,7,11]$, the program computes the Pedersen hash and prints the result:
>```
>The Pedersen hash is: 8
## How It Works:
- **Modular Exponentiation:** The function `mod_exp` computes each generator raised to the power of the corresponding value modulo the group modulus.
- **Pedersen Hash Calculation:** The result of each exponentiation is multiplied together modulo `p`. This result is the Pedersen hash.
## Contributing
  - If you intend to contribute to this project, fork the repository and make a pull request.

## Usage
- To use this code, you can clone the repository.
- Compile the Rust code using cargo:
>```
>cargo build
>cargo run

## Acknowledgments
- Rust
### Clone the repository or copy the source code into a Rust project.
```bash
   git clone https://github.com/Computing_Pedersen_Hash_Using_Rust.git
   cd Computing_Pedersen_Hash_Using_Rust
