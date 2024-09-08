enum BinOps {
    And,
    Or,
    Not,
    /* We assume other operators
    (implies, xor, ...) could be
    constructed with those 3. */
}

struct Variable;

struct Predicate {
    variables: Vec<Variable>,
    body: Vec<Box<Self>>,
}

fn main() {}
