# Asynmov

Generate large corpora of synthetic data for world building. Asynmov is a Rust-core library with Python (via PyO3) and Node.js (via napi-rs) surfaces. Produce millions of coherent entities from a TOML config — the raw material for fiction, games, simulation, and LLM training data — with parallel generation and zero Python overhead on the hot path.

## Features

- **Configuration-first** — describe the world in a TOML file; Asynmov handles the rest
- **Columnar output** — entities are generated as typed columns and returned as a [Polars](https://pola.rs) DataFrame (Python) or JSON rows (Node.js)
- **Full type support** — `uniform_int`, `uniform_float`, `normal`, and `choice` distributions with `bool`, `int`, `float`, and `str` values
- **Deterministic** — supply a seed and get identical output every time, regardless of thread scheduling
- **Parallel core** — Rayon-parallel generation across all entities; each column is independent
- **Multiple output formats** — Parquet (default), JSON, JSONL, and CSV
- **CLI included** — `asynmov generate` and `asynmov validate` for scripted pipelines

## Installation

### Python

```sh
pip install asynmov
```

Requires Python 3.11–3.13. Pre-built wheels for Linux (x86\_64, aarch64, musl), macOS (x86\_64, arm64), and Windows (x86\_64). No Rust toolchain required.

### Node.js

```sh
npm install @asynmov/asynmov
```

Pre-built native modules for the same platform matrix.

## Quick start

### Python

```python
from asynmov import World

world = World.from_config("my_world.toml")
df = world.generate(scale=1000).entities  # polars.DataFrame

# Query directly
print(df.filter(df["occupation_class"] == "farmer").select(["age", "wealth_score"]))

# Or export
world.generate(scale=10_000).export("output/", format="parquet")
```

### Node.js

```js
const { generateFromToml, validateConfig } = require('@asynmov/asynmov')
const { readFileSync } = require('fs')

const toml = readFileSync('my_world.toml', 'utf8')
validateConfig(toml)  // throws on invalid config

const rows = JSON.parse(generateFromToml(toml, 1000))
console.log(rows[0])
// { id: 0, age: 39.1, gender: 'female', occupation_class: 'skilled_trade', ... }
```

## World config

```toml
[world]
name = "The Gilded Republic"
seed = 42

[[attributes]]
name = "age"
type = "normal"
mean = 35.0
std_dev = 14.0
min = 0.0
max = 90.0

[[attributes]]
name = "gender"
type = "choice"
values = ["male", "female"]
weights = [51.0, 49.0]

[[attributes]]
name = "literacy"
type = "choice"
values = [true, false]   # bool columns are natively supported
weights = [70.0, 30.0]
```

See [`docs/examples/american_historical.toml`](docs/examples/american_historical.toml) for a full example.

**Supported distribution types:**

| Type | Fields |
|------|--------|
| `uniform_int` | `low`, `high` |
| `uniform_float` | `low`, `high` |
| `normal` | `mean`, `std_dev`, `min?`, `max?` |
| `choice` | `values` (str/int/float/bool), `weights?` |

## CLI

```sh
# Generate a corpus
asynmov generate --config my_world.toml --scale 5000 --output output/ --format parquet

# Validate a config without generating
asynmov validate my_world.toml
```

Output formats: `parquet` (default), `json`, `jsonl`, `csv`.

## Development

Requires a Rust toolchain ([rustup.rs](https://rustup.rs)).

```sh
# Python
pip install maturin polars pytest
maturin build --release --manifest-path crates/asynmov-py/Cargo.toml --out dist
pip install --no-index --find-links dist asynmov
pytest

# Node.js
cd npm
npm install
npm run build:debug
node --test
```

### Workspace layout

```
Cargo.toml                  # workspace root
crates/
  asynmov-core/             # pure Rust — all logic, no FFI
    src/config.rs           # TOML parsing → WorldConfig
    src/rng.rs              # seed derivation
    src/generators/
      entities.rs           # columnar parallel generation
  asynmov-py/               # PyO3 cdylib → asynmov._core
  asynmov-node/             # napi-rs cdylib → @asynmov/asynmov
npm/                        # Node package + per-platform dirs
  linux-x64-gnu/
  linux-x64-musl/
  linux-arm64-gnu/
  darwin-x64/
  darwin-arm64/
  win32-x64-msvc/
python/asynmov/             # thin Python surface (World, Corpus, CLI)
```

## Documentation

Design and architecture documentation lives in [`docs/`](docs/).

## License

MIT — see [LICENSE](LICENSE).
