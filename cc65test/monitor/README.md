# Build image

```bash
make
```

# Run in r6502

```bash
cargo run -p r6502 run .\cc65test\monitor\monitor.sim65 --trace --reset --cycles --emu apple1
```

Or:

```bash
cargo run -p r6502 run .\cc65test\monitor\monitor.sim65 --reset --cycles --emu apple1
```
