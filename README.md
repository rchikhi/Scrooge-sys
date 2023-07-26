# Scrooge-sys

Rust bindings for the fast DNA alignment library Scrooge: https://github.com/CMU-SAFARI/Scrooge

See https://github.com/rchikhi/rust-alignbench for an example usage

# Command used to generate the bindings:

```
cd src && bindgen ../Scrooge/src/genasm_cpu.cpp  --allowlist-function "genasm.*" -- -x c++
```

