use std::marker::PhantomData;

use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::{AssignedCell, Layouter, Value},
    plonk::{Advice, Assigned, Column, ConstraintSystem, Constraints, Error, Expression, Selector},
    poly::Rotation,
};

mod table;
use table::RangeTableConfig;

//LOOKUP_RANGE > RANGE
#[derive(Debug, Clone)]
struct RangeCheckConfig<F: FieldExt, const RANGE: usize, const LOOKUP_RANGE: usize> {
    value: Column<Advice>,
    q_range_check: Selector,
    q_lookup: Selector,
    table: RangeTableConfig<F, LOOKUP_RANGE>,
    _marker: PhantomData<F>,
}

impl<F: FieldExt, const RANGE: usize, const LOOKUP_RANGE: usize>
    RangeCheckConfig<F, RANGE, LOOKUP_RANGE>
{
    fn configure(meta: &mut ConstraintSystem<F>, value: Column<Advice>) -> Self {
        let q_range_check = meta.selector();
        //toggles the lookup argument
        //selector optimisation can hand up resulting in non-binary values, which we don't want when using these as toggle values
        let q_lookup = meta.complex_selector();

        let table = RangeTableConfig::configure(meta);

        let config =
            Self { q_range_check, value, q_lookup, table: table.clone(), _marker: PhantomData };

        meta.create_gate("range check", |meta| {
            //when querying a selector we don't specific the rotation cause we always query at the current rotation, but advices are then indexed around the current selector location
            let q_range_check = meta.query_selector(q_range_check);
            let value = meta.query_advice(value, Rotation::cur());
            //for a value v and a range R, check that v < R
            //v * (1 - v) * (2 - v) * (3 - v) ... * ( R - 1 - v) = 0
            let range_check = |range: usize, value: Expression<F>| {
                (0..range).fold(value.clone(), |expr, i| {
                    expr * (Expression::Constant(F::from(i as u64)) - value.clone())
                })
            };
            //multiply each constraint by its given selector
            Constraints::with_selector(q_range_check, [("range check", range_check(RANGE, value))])
        });
        //range check lookup
        meta.lookup(|meta| {
            let q_lookup = meta.query_selector(q_lookup);
            let value = meta.query_advice(value, Rotation::cur());

            vec![(q_lookup * value, table.value)]
        });

        config
    }

    fn assign(
        &self,
        mut layouter: impl Layouter<F>,
        value: Value<Assigned<F>>,
        range: usize,
    ) -> Result<(), Error> {
        assert!(range <= LOOKUP_RANGE);

        if range < RANGE {
            layouter.assign_region(
                || "Assign value",
                |mut region| {
                    let offset = 0;
                    //enable q_range_check
                    self.q_range_check.enable(&mut region, offset)?;

                    region.assign_advice(|| "Assign value", self.value, offset, || value)?;

                    Ok(())
                },
            )
        } else {
            layouter.assign_region(
                || "assign value for lookup range check",
                |mut region| {
                    let offset = 0;

                    //enable q_range_check
                    self.q_lookup.enable(&mut region, offset)?;

                    region.assign_advice(|| "Assign value", self.value, offset, || value)?;

                    Ok(())
                },
            )
        }
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
    struct MyCircuit<F: FieldExt, const RANGE: usize, const LOOKUP_RANGE: usize> {
        value: Value<Assigned<F>>,
        large_value: Value<Assigned<F>>,
    }

    impl<F: FieldExt, const RANGE: usize, const LOOKUP_RANGE: usize> Circuit<F>
        for MyCircuit<F, RANGE, LOOKUP_RANGE>
    {
        type Config = RangeCheckConfig<F, RANGE, LOOKUP_RANGE>;
        type FloorPlanner = V1;

        fn without_witnesses(&self) -> Self {
            Self::default()
        }

        fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
            let value = meta.advice_column();
            RangeCheckConfig::configure(meta, value)
        }

        fn synthesize(
            &self,
            config: Self::Config,
            mut layouter: impl Layouter<F>,
        ) -> Result<(), Error> {
            config.table.load(&mut layouter)?;

            config.assign(layouter.namespace(|| "Assign value"), self.value, RANGE)?;
            config.assign(
                layouter.namespace(|| "Assign larger value"),
                self.large_value,
                LOOKUP_RANGE,
            )?;

            Ok(())
        }
    }

    #[test]
    fn test_range_check_1() {
        //mumber of rows, 2^k, important to note that some rows are reserved for random values used in constructing the tables
        let k = 9;
        const RANGE: usize = 8; // 3-bit value
        const LOOKUP_RANGE: usize = 256; // 8-bit value

        // Successful cases
        for i in 0..RANGE {
            let circuit = MyCircuit::<Fp, RANGE, LOOKUP_RANGE> {
                value: Value::known(Fp::from(i as u64).into()),
                large_value: Value::known(Fp::from(i as u64).into()),
            };

            let prover = MockProver::run(k, &circuit, vec![]).unwrap();
            prover.assert_satisfied();
        }

        // // Out-of-range `value = 8`
        // {
        //     let circuit = MyCircuit::<Fp, RANGE, LOOKUP_RANGE> {
        //         value: Value::known(Fp::from(RANGE as u64).into()),
        //     };
        //     let prover = MockProver::run(k, &circuit, vec![]).unwrap();
        //     assert_eq!(
        //         prover.verify(),
        //         Err(vec![VerifyFailure::ConstraintNotSatisfied {
        //             constraint: ((0, "range check").into(), 0, "range check").into(),
        //             location: FailureLocation::InRegion {
        //                 region: (0, "Assign value").into(),
        //                 offset: 0
        //             },
        //             cell_values: vec![(((Any::Advice, 0).into(), 0).into(), "0x8".to_string())]
        //         }])
        //     );
        // }
    }
}
