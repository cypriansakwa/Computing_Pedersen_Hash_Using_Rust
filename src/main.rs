fn mod_exp(base: u32, exp: u32, modulus: u32) -> u32 {
    let mut result = 1;
    let mut base = base % modulus;
    let mut exp = exp;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp = exp >> 1;
        base = (base * base) % modulus;
    }
    result
}

fn pedersen_hash(g: &[u32], x: &[u32], modulus: u32) -> u32 {
    let mut hash = 1;
    for (gi, xi) in g.iter().zip(x.iter()) {
        hash = (hash * mod_exp(*gi, *xi, modulus)) % modulus;
    }
    hash
}

fn main() {
    // The group modulus
    let p = 13;

    // The generators of Z_13*
    let generators = vec![2,6,7];

    // The vector (3, 7, 11)
    let values = vec![3,7,11];

    // Compute the Pedersen hash
    let hash = pedersen_hash(&generators, &values, p);

    println!("The Pedersen hash is: {}", hash);
}

