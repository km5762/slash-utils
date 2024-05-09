# /UTILS

Project Description

### TASKS

- [x] WASM Compatibility
  - [ ] Convert all state to 1d array instead of 2d

### COMPLETED ✓

- [x] AES key schedule
- [x] AES cipher function
  - [x] shift_rows()
  - [x] add_round_key()
  - [x] mix_columns()
- [x] AES inverse cipher
  - [x] inverse_shift_rows()
  - [x] inverse_sub_bytes()
  - [x] inverse_mix_columns()
- [ ] Return intermediate values from cipher functions
  - [x] In cipher()
  - [ ] In inv_cipher()
