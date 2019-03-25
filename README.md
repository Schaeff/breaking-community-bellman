# breaking-community-bellman
A minimal example of regression in bellman 0.2.0

`before` and `after` have the same contents, except `Cargo.lock` that was removed in `after`.
`before` builds fine.
`after` fails with
```bash
error[E0277]: the trait bound `E: pairing::Engine` is not satisfied
 --> src/lib.rs:7:17
  |
7 | impl<E: Engine> Circuit<E> for Dummy {
  |                 ^^^^^^^^^^ the trait `pairing::Engine` is not implemented for `E`
  |
  = help: consider adding a `where E: pairing::Engine` bound

error[E0277]: the trait bound `E: pairing::Engine` is not satisfied
  --> src/lib.rs:8:5
   |
8  | /     fn synthesize<CS: ConstraintSystem<E>>(self, _cs: &mut CS) -> Result<(), SynthesisError> {
9  | |         unimplemented!()
10 | |     }
   | |_____^ the trait `pairing::Engine` is not implemented for `E`
   |
   = help: consider adding a `where E: pairing::Engine` bound
   = note: required by `bellman::cs::ConstraintSystem`
