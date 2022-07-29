## Testing

Adjust `.cargo/config.toml` and `memory.x` for your target.

```bash
cargo test -p testsuite
```

## ASM

pre-process it:

```bash
arm-none-eabi-gcc -O0 -ffunction-sections -fdata-sections -g -fno-omit-frame-pointer -mthumb -march=armv7e-m -Wall -Wextra -std=c11 -march=armv7e-m -c P256-Cortex-M4/p256-cortex-m4-asm-gcc.S -E > asm.s
```
