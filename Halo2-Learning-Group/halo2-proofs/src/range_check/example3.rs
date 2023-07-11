use std::marker::PhantomData;

use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::{Layouter, Value},
    plonk::{Advice, Assigned, Column, ConstraintSystem, Error, Expression, Selector},
    poly::Rotation,
};

mod table;
use table::RangeTableConfig;

//LOOKUP_RANGE > RANGE
#[derive(Debug, Clone)]
struct RangeCheckConfig<F: FieldExt, const NUM_BITS: usize, const RANGE: usize> {
    value: Column<Advice>,
    num_bits: Column<Advice>,
    q_range_check: Selector,
    q_lookup: Selector,
    table: RangeTableConfig<F, NUM_BITS, RANGE>,
    _marker: PhantomData<F>,
}

impl<F: FieldExt, const NUM_BITS: usize, const RANGE: usize> RangeCheckConfig<F, NUM_BITS, RANGE> {
    fn configure(
        meta: &mut ConstraintSystem<F>,
        value: Column<Advice>,
        num_bits: Column<Advice>,
    ) -> Self {
        let q_range_check = meta.selector();
        //toggles the lookup argument
        //selector optimisation can hand up resulting in non-binary values, which we don't want when using these as toggle values
        let q_lookup = meta.complex_selector();

        let table = RangeTableConfig::configure(meta);

        let config = Self {
            value,
            num_bits,
            q_range_check,
            q_lookup,
            table: table.clone(),
            _marker: PhantomData,
        };

        //range check lookup
        meta.lookup(|meta| {
            let q_lookup = meta.query_selector(q_lookup);
            let num_bits = meta.query_advice(num_bits, Rotation::cur());
            let value = meta.query_advice(value, Rotation::cur());

            let not_q_lookup = Expression::Constant(F::one()) - q_lookup.clone();
            let default_num_bits = Expression::Constant(F::one()); // 1-bit
            let default_value = Expression::Constant(F::zero()); // 0 is a 1-bit value

            let num_bits_expr =
                q_lookup.clone() * num_bits + not_q_lookup.clone() * default_num_bits;
            let value_expr = q_lookup * value + not_q_lookup * default_value;

            vec![(num_bits_expr, table.num_bits), (value_expr, table.value)]
        });

        config
    }

    fn assign(
        &self,
        mut layouter: impl Layouter<F>,
        value: Value<Assigned<F>>,
        num_bits: Value<u8>,
    ) -> Result<(), Error> {
        layouter.assign_region(
            || "assign value for lookup range check",
            |mut region| {
                let offset = 0;

                //enable q_range_check
                self.q_lookup.enable(&mut region, offset)?;

                region.assign_advice(
                    || "Assign num_bits",
                    self.num_bits,
                    offset,
                    || {
                        // Convert num_bits to Assigned<F>
                        num_bits.map(|v| F::from(v as u64))
                    },
                )?;

                region.assign_advice(|| "Assign value", self.value, offset, || value)?;

                Ok(())
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use halo2_proofs::{circuit::floor_planner::V1, dev::MockProver, pasta::Fp, plonk::Circuit};

    use super::*;

    #[derive(Default)]
    struct MyCircuit<F: FieldExt, const NUM_BITS: usize, const RANGE: usize> {
        value: Value<Assigned<F>>,
        num_bits: Value<u8>,
    }

    impl<F: FieldExt, const NUM_BITS: usize, const RANGE: usize> Circuit<F>
        for MyCircuit<F, NUM_BITS, RANGE>
    {
        type Config = RangeCheckConfig<F, NUM_BITS, RANGE>;
        type FloorPlanner = V1;

        fn without_witnesses(&self) -> Self {
            Self::default()
        }

        fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
            let value = meta.advice_column();
            let num_bits = meta.advice_column();
            RangeCheckConfig::configure(meta, value, num_bits)
        }

        fn synthesize(
            &self,
            config: Self::Config,
            mut layouter: impl Layouter<F>,
        ) -> Result<(), Error> {
            config.table.load(&mut layouter)?;

            // config.assign(layouter.namespace(|| "Assign value"), self.value, Value::known(F::zero().into()), RANGE)?;
            config.assign(layouter.namespace(|| "Assign value"), self.value, self.num_bits)?;

            Ok(())
        }
    }

    #[test]
    fn test_range_check_3() {
        //mumber of rows, 2^k, important to note that some rows are reserved for random values used in constructing the tables
        let k = 9;
        const NUM_BITS: usize = 8; // 8-bit value
        const RANGE: usize = 256; // 8-bit value

        for numbits in 1u8..=NUM_BITS.try_into().unwrap() {
            for value in (1 << (numbits - 1))..(1 << numbits) {
                let circuit = MyCircuit::<Fp, NUM_BITS, RANGE> {
                    value: Value::known(Fp::from(value).into()),
                    num_bits: Value::known(numbits),
                };

                let prover = MockProver::run(k, &circuit, vec![]).unwrap();
                prover.assert_satisfied();
            }
        }
    }
}
