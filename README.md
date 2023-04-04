### KSP Config Tool

This is a rust "script" for removing parts/modules/resources from KSP1 craft files.

Its primary purpose is to clean up invalid stuff from 'stock' crafts for use in RSS/RO/RP-1.

### Installation

- Install rust via https://www.rust-lang.org/tools/install
- Install kspconfigtool:

```
cargo install kspconfigtool
```

### Usage

For example:

```
kspconfigtool remove module TweakScale Ships/VAB/*.craft
kspconfigtool remove remove module ModuleColliderHelper Ships/VAB/*.craft
kspconfigtool remove resource LiquidFuel Ships/VAB/*.craft
kspconfigtool remove resource Oxidizer Ships/VAB/*.craft
kspconfigtool remove part sensorAccelerometer Ships/VAB/*.craft
```

It can also be used to read and write a craft file and 'clean' it up:

```
kspconfigtool clean Ships/VAB/*.craft
```

### Behavior

- It creates sequentially numbered backups: `.orig1`, `.orig2`, etc.
- It preserves dos/unix line endings based on whatever it finds in the original file.
- It uses tabs instead of spaces (like KSP itself appears to).
- If there is nothing to do it does nothing (no backup file, etc).

### Futures

- Maybe extend the parser to read all the broken config files that the KSP parser reads.
- Maybe flags for --dry-run, converting tabs to spaces, and selecting dos-vs-unix.
- Maybe add a proper rust serde serializer/deserializer.
- Has zero tests, needs tests.

### Out Of Scope

- Rewriting ModuleManager in Rust (no thanks).
- KSP2 (just use JSON).

