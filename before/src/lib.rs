use pairing::Engine;

use bellman::{Circuit, ConstraintSystem, SynthesisError};

pub struct Dummy;

impl<E: Engine> Circuit<E> for Dummy {
    fn synthesize<CS: ConstraintSystem<E>>(self, _cs: &mut CS) -> Result<(), SynthesisError> {
        unimplemented!()
    }
}
