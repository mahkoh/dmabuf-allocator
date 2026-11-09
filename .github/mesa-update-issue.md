- [ ] Update the `mesa/mesa` submodule to the `{{tag}}` tag.
- [ ] Run the `mesa/build-mesa.sh` script.
- [ ] Check whether the release adds drivers, and implement the new drivers.
- [ ] Check whether the behavior of the existing drivers changed, including
      support for new formats or modifiers.
- [ ] Run the `cargo run -p codegen --bin intel_devices` command.
- [ ] Run the `cargo run --release -p mesa-testgen` command.
- [ ] Run the `cargo run --release -p generated-tests` command.
- [ ] Run the `cargo test` command.
