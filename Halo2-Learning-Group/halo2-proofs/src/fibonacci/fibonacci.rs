use std::marker::PhantomData;

use halo2_proofs::{
    arithmetic::FieldExt, circuit::*, dev::MockProver, pasta::Fp, plonk::*, poly::Rotation,
};

/// One configuration for this gadget could look like:
///
///     |      a     |        b      |        c       |      s       |
///     -------------------------------------------------------------
///     |       1     |       1       |       2       |      1       |
///     |       1     |       2       |       3       |      1       |
///     |       2     |       3       |       5       |      1       |
///     |       3     |       5       |       8       |      1       |
///     |     z_C     |       0       |      ...      |              |
///
///
/// Another configuration for this gadget could look like:
///
///     |      a     |        s      
///     ------------------------------
///     |       1     |       1     
///     |       1     |            
///     |       2     |             
///     |       1     |       1      
///     |       2     |         
///     |       3     |         
///     |       2     |       1  
///     |       3     |         
///     |       5     |         
///     |       3     |       1  

#[derive(Debug, Clone)]
struct FibonacciConfig<F: FieldExt, const FIBONACCI_NUMBER: usize> {
    advice: Column<Advice>,
    instance: Column<Instance>,
    selector: Selector,
    _marker: PhantomData<F>,
}

impl<F: FieldExt, const FIBONACCI_NUMBER: usize> FibonacciConfig<F, FIBONACCI_NUMBER> {
    fn configure(meta: &mut ConstraintSystem<F>) -> Self {
        //allocating our only advice column
        let advice = meta.advice_column();

        //allocating our only instance column
        let instance = meta.instance_column();

        //allow for permutation within the column
        meta.enable_equality(advice);
        //We need to allow instances too as these will get copied over;
        //specifically, the first two public inputs will constitute our first two rows
        meta.enable_equality(instance);

        //allocate simple selector
        let selector = meta.selector();

        meta.create_gate("Fibonacci", |meta| {
            let a = meta.query_advice(advice, Rotation::cur());
            let b = meta.query_advice(advice, Rotation::next());
            let c = meta.query_advice(advice, Rotation(2));

            let s = meta.query_selector(selector);
            // s * (a + b - c)

            vec![s * (a + b - c)]
        });

        Self { advice, instance, selector, _marker: PhantomData }
    }

    fn assign(&self, mut layouter: impl Layouter<F>) -> Result<AssignedCell<F, F>, Error> {
        layouter.assign_region(
            || "Fibonacci Column",
            |mut region| {
                //enabling selector at first row, index 0
                self.selector.enable(&mut region, 0)?;

                //Assigning first fibo numbers
                let mut a_cell = region.assign_advice_from_instance(
                    || "Assign first Numbe: 1",
                    self.instance,
                    0,
                    self.advice,
                    0,
                )?;
                let mut b_cell = region.assign_advice_from_instance(
                    || "Assign Second Numbe: 1",
                    self.instance,
                    1,
                    self.advice,
                    1,
                )?;

                for row in 2..FIBONACCI_NUMBER {
                    //remember selector is only switched on every three
                    if row < FIBONACCI_NUMBER - 2 {
                        self.selector.enable(&mut region, row)?;
                    }

                    let c_val = a_cell.value().and_then(|a| b_cell.value().map(|b| *a + *b));

                    let c_cell = region.assign_advice(|| "c_value", self.advice, row, || c_val)?;

                    a_cell = b_cell;
                    b_cell = c_cell;
                }
                Ok(b_cell)
            },
        )
    }

    pub fn expose_public(
        &self,
        mut layouter: impl Layouter<F>,
        cell: AssignedCell<F, F>,
        row: usize,
    ) -> Result<(), Error> {
        layouter.constrain_instance(cell.cell(), self.instance, row)
    }
}

#[cfg(test)]
mod tests {
    use halo2_proofs::{
        circuit::floor_planner::V1,
        dev::{FailureLocation, MockProver, VerifyFailure},
        pasta::Fp,
        plonk::{Any, Circuit},
    };

    use super::*;

    #[derive(Default)]
    struct MyCircuit<F: FieldExt, const FIBONACCI_NUMBER: usize> {
        _marker: PhantomData<F>,
    }

    impl<F: FieldExt, const FIBONACCI_NUMBER: usize> Circuit<F> for MyCircuit<F, FIBONACCI_NUMBER> {
        type Config = FibonacciConfig<F, FIBONACCI_NUMBER>;
        type FloorPlanner = V1;

        fn without_witnesses(&self) -> Self {
            println!("no witness");
            Self::default()
        }

        fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
            FibonacciConfig::configure(meta)
        }

        fn synthesize(
            &self,
            config: Self::Config,
            mut layouter: impl Layouter<F>,
        ) -> Result<(), Error> {
            let out_cell = config.assign(layouter.namespace(|| "Assign value"))?;

            config.expose_public(layouter.namespace(|| "out"), out_cell, 2)?;

            Ok(())
        }
    }

    #[test]
    fn test_fibonacci() {
        let k = 4;

        let a = Fp::from(1);
        let b = Fp::from(1);
        let out = Fp::from(55);

        const FIBONACCI_NUMBER: usize = 10;

        let circuit = MyCircuit::<Fp, FIBONACCI_NUMBER> { _marker: PhantomData };

        let public_input = vec![a, b, out];

        let prover = MockProver::run(k, &circuit, vec![public_input]).unwrap();

        prover.assert_satisfied();
    }

    #[cfg(feature = "dev-graph")]
    #[test]
    fn plot_fibonacci() {
        use plotters::prelude::*;

        let root = BitMapBackend::new("fibonacci-layout.png", (1024, 3096)).into_drawing_area();
        root.fill(&WHITE).unwrap();
        let root = root.titled("Fibonacci Layout", ("sans-serif", 60)).unwrap();

        const FIBONACCI_NUMBER: usize = 10;

        let circuit = MyCircuit::<Fp, FIBONACCI_NUMBER> { _marker: PhantomData };

        halo2_proofs::dev::CircuitLayout::default().render(4, &circuit, &root).unwrap();
    }
}
