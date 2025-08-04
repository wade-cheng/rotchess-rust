# rotchess-core

Core library code for headless rotchess logic in Rust.

This module provides functionality for a `RotchessEmulator` to modify
and record state over time as it is fed information in the form of
events. External code should read the board's state and
draw it for the user. This crate is not responsible for any drawing of state.
It does, however, provide functions that help the drawing program understand
what to draw.

## Performance Notes

I do some testing via `cargo flamegraph`, with [this](https://github.com/wade-cheng/rotchess-rust)
as the frontend.

### b3119b6a6ad47a75f038033681d355dda832b202

This is the baseline commit. [`flamegraph`](doc_assets/flamegraph_init_auxiliary_data_b3119b6a6ad47a75f038033681d355dda832b202.svg)

### cff26bed34f95e6833fe8cc68c87b2068d49c1ec

I started using `update_capmove_points_unchecked` instead of initializing the whole piece whenever possible. This made things slower. [`flamegraph`](doc_assets/flamegraph_update_capmove_cff26bed34f95e6833fe8cc68c87b2068d49c1ec.svg)
