use halo2_base::gates::{GateChip, GateInstructions};
use halo2_base::utils::ScalarField;
use halo2_base::AssignedValue;
#[allow(unused_imports)]
use halo2_base::{
    Context,
    QuantumCell::{Constant, Existing, Witness},
};
use serde::{Deserialize, Serialize};
use std::env::var;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CircuitInput {
    pub a: String,
    pub b: String,
    pub out: String,
}

// this algorithm takes a public input x, computes x^2 + 72, and outputs the result as public output
pub fn compute_fibonacci<F: ScalarField>(
    ctx: &mut Context<F>,
    input: CircuitInput,
    make_public: &mut Vec<AssignedValue<F>>,
) {
    let a = F::from_str_vartime(&input.a).expect("deserialize field element should not fail");
    let b = F::from_str_vartime(&input.b).expect("deserialize field element should not fail");
    // let fibonacci_number = F::from_str_vartime(&input.fibonacci_number).expect("deserialize field element should not fail");
    let fibonacci_number = var("FIBONACCI_NUMBER")
        .unwrap_or_else(|_| panic!("fibonacci_number not set"))
        .parse()
        .unwrap();
    let out = F::from_str_vartime(&input.out).expect("deserialize field element should not fail");

    // first we load a number `x` into as system, as a "witness"
    let mut a = ctx.load_witness(a);
    let mut b = ctx.load_witness(b);

    // by default, all numbers in the system are private
    // we can make it public like so:
    make_public.push(a);
    make_public.push(b);

    // create a Gate chip that contains methods for basic arithmetic operations
    let gate = GateChip::<F>::default();

    for _row in 2..fibonacci_number {
        let c = gate.add(ctx, a, b);
        a = b;
        b = c;
    }

    let out = ctx.load_witness(out);
    make_public.push(out);

    ctx.constrain_equal(&out, &b);

    println!("a: {:?}", a.value());
    println!("b: {:?}", b.value());
    println!("fibonacci_number: {:?}", fibonacci_number);
    println!("out: {:?}", out);
    assert_eq!(*out.value(), *b.value());
}
