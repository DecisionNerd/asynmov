# Asynmov

Generate large corpora of synthetic data for world building. Asynmov pairs a Python API with a high-performance Rust core so you can produce millions of coherent entities, events, and relationships — the raw material for fiction, games, simulation, and LLM training data — without hitting a performance wall.

## Features

- **Entity generation** — characters with internally consistent attributes driven by world configuration
- **Event generation** — causally ordered life events (birth, migration, occupation, death, and more)
- **Relationship graphs** — social, family, and professional networks across the population
- **Configuration-first** — describe the world in a TOML file; Asynmov handles the rest
- **Deterministic** — supply a seed and get identical output every time
- **Multiple output formats** — JSON, JSONL, and CSV, ready to load into any downstream tool
- **CLI included** — `asynmov generate` and `asynmov validate` for scripted pipelines

## Installation

```sh
pip install asynmov
```

Pre-built wheels are provided for Linux (x86\_64, aarch64), macOS (x86\_64, arm64), and Windows (x86\_64). No Rust toolchain required to use the package.

## Quick start

```python
from asynmov import World

world = World.from_config("my_world.toml")
corpus = world.generate(scale=1000)
corpus.export("output/", format="json")
```

A minimal config:

```toml
[world]
name = "The Gilded Republic"
era = "1880-1920"
setting = "american_historical"
seed = 42

[population]
size = 1000
regions = ["northeast", "midwest", "south"]
```

## CLI

```sh
# Generate a corpus
asynmov generate --config my_world.toml --scale 5000 --output output/ --format jsonl

# Validate a config without generating
asynmov validate my_world.toml
```

## Development

Requires a Rust toolchain ([rustup.rs](https://rustup.rs)) and [maturin](https://github.com/PyO3/maturin).

```sh
pip install maturin
maturin develop      # compile Rust extension and install into current venv
pytest               # run tests
```

## Documentation

Full design and architecture documentation lives in [`docs/`](docs/).

## License

MIT — see [LICENSE](LICENSE).
