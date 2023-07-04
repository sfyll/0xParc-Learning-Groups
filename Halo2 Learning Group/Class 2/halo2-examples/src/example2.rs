use std::marker::PhantomData;

use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::*,
    plonk::*,
    pasta::Fp, 
    dev::MockProver, 
    poly::Rotation,
};

#[derive(Debug, Clone)]
struct Acell<F: FieldExt>(AssignedCell<F, F>);

#[derive(Debug, Clone)]
struct FiboConfig{
    pub advice: Column<Advice>,
    pub selector: Selector,
    pub instance: Column<Instance>
}

struct FiboChip<F: FieldExt> {
    config: FiboConfig, 
    _marker: PhantomData<F>,
}

impl<F: FieldExt> FiboChip<F> {
    fn construct(config: FiboConfig) -> Self {
        Self {config, _marker: PhantomData}
    }

    fn configure(meta: &mut ConstraintSystem<F>,
                advice: Column<Advice>,
                instance: Column<Instance>,
            ) -> FiboConfig {
        let selector = meta.selector();

        meta.enable_equality(advice);
        meta.enable_equality(instance);

        meta.create_gate("add", |meta: &mut VirtualCells<F>| {
            // advice | selector
            //   a |    s
            //   b | 
            //   c | 
            let s= meta.query_selector(selector);
            let a = meta.query_advice(advice, Rotation::cur()); //rotation is indexed to selector s position
            let b = meta.query_advice(advice, Rotation::next());
            let c = meta.query_advice(advice, Rotation(2));
            vec![s * ( a + b - c)]
        });

        FiboConfig { 
            advice, 
            selector,
            instance
        }
    }

    fn assign(&self, mut layouter: impl Layouter<F>, nrows: usize)
    -> Result<AssignedCell<F, F>, Error> {
        layouter.assign_region(||"entire fibonacci table", 
        |mut region| {
            self.config.selector.enable(&mut region, 0)?;

            let mut a_cell = region.assign_advice_from_instance(
                ||"1",
                self.config.instance,
                0,
                self.config.advice,
                0
            )?;

            let mut b_cell = region.assign_advice_from_instance(
                ||"1",
                self.config.instance,
                1,
                self.config.advice,
                1
            )?;
            
            for row in 2..nrows {
                if row < nrows - 2 {
                    //belows mean that selector will be swiched on and as such a value should be expected at this row
                    self.config.selector.enable(&mut region, row)?;
                }
                let c_val = a_cell.value().and_then(|a| {
                    b_cell.value().map(|b| *a + *b)
                });

                let c_cell = region.assign_advice(
                    ||"advice", 
                    self.config.advice, 
                    row, 
                    ||c_val.ok_or(Error::Synthesis),
                )?;

                a_cell = b_cell;
                b_cell = c_cell;
            }

            Ok(b_cell)
        }
    )}

        pub fn expose_public(&self,
            mut layouter: impl Layouter<F>,
            cell: AssignedCell<F, F>,
            row: usize) -> Result<(), Error>{
                layouter.constrain_instance(cell.cell(), 
                self.config.instance,
                row)
            }
}

#[derive(Default)]
struct MyCircuit<F> {
    pub a: Option<F>,
    pub b: Option<F>,
}

impl<F: FieldExt> Circuit<F> for MyCircuit<F> {
    type Config = FiboConfig;
    type FloorPlanner = SimpleFloorPlanner;
    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        let advice =  meta.advice_column();
        let instance = meta.instance_column();
        FiboChip::configure(meta, advice, instance)
    }

    fn synthesize(&self, config: Self::Config, mut layouter: impl Layouter<F>) -> Result<(), Error> {
        let chip =  FiboChip::construct(config);
        let out_cell = chip.assign(
            layouter.namespace(||"entire table"),
            10
        )?;

        chip.expose_public(layouter.namespace(||"out"), out_cell, 2)?;

        Ok(())
    }
}


fn main() {
    let k = 4;

    let a = Fp::from(1);
    let b = Fp::from(1);
    let out = Fp::from(55);

    let circuit = MyCircuit{
        a: Some(a),
        b: Some(b),
    };

    let public_input = vec![a, b, out];

    let prover = MockProver::run(k, &circuit, vec![public_input.clone()]).unwrap();

    prover.assert_satisfied();


    println!("Hello, world!");
}
