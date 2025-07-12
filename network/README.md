Requires nightly build of Rust until RFC3324 is stabilized (see
https://github.com/rust-lang/rust/issues/65991).

```
rustup install nightly
cargo +nightly run
```

```
└── src
    ├── lib.rs      - network, fprop, bprop
    ├── data        - mnist retrieval & processing
    ├── math        - activation & loss functions
    ├── parameters  - read/write, randomize
    ├── training    - gradient descent
    └── util        - data structures, test
```
