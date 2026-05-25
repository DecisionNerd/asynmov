# Experience

This document describes the intended user and developer experience — the feel of the API and tooling, not the implementation.

## User Experience

### Getting started in under five minutes

A new user should be able to install the package, configure a world, and generate a corpus without reading more than the README.

```python
from asynmov import World

world = World.from_config("my_world.toml")
corpus = world.generate(scale=1000)
corpus.export("output/", format="json")
```

### Configuration-first design

The entry point is a world configuration, not a collection of individual generator calls. Users describe *what* they want the world to be; Asynmov decides *how* to generate consistent data for it.

```toml
# my_world.toml
[world]
name = "The Gilded Republic"
era = "1880-1920"
setting = "american_historical"
seed = 42

[population]
size = 5000
regions = ["northeast", "midwest", "south"]
```

### Deterministic by default

Supplying a `seed` produces identical output across runs. Omitting it uses a random seed that is logged so results can be reproduced.

### Progressive disclosure

Simple use requires simple configuration. Advanced users can override individual generators, supply custom rule sets, or hook into the generation pipeline to inject domain-specific logic — without that complexity surfacing for basic use.

### Output is boring on purpose

Outputs are plain structured data: JSON objects, CSV rows, JSONL streams. No proprietary format. No mandatory schema registration. The assumption is that users will load this data into their own systems.

---

## Developer Experience

### Python is the public surface

All user-facing APIs are Python. The Rust backend is an implementation detail. Developers contributing features write Python interfaces; performance-critical internals may be implemented in Rust.

### Type-annotated throughout

All public functions carry type annotations. A `py.typed` marker is included. IDE autocompletion should work without extra setup.

### Errors are actionable

Errors should tell the user what was wrong and how to fix it, not just what failed. Configuration errors are caught at load time, not mid-generation.

### CLI parity with the Python API

Everything achievable via the Python API should be achievable from the command line:

```sh
asynmov generate --config my_world.toml --scale 1000 --output output/
```

### Fast iteration

The development loop (edit → test → observe output) should be fast. The Rust backend compiles once; Python changes do not require recompilation.
