# Architecture

## Overview

Asynmov is a **Rust-first** project. All logic — config parsing, seed management, and data generation — lives in `asynmov-core`. Language surfaces (Python via PyO3, Node.js via napi-rs) are thin bindings that call into core and translate the result into idiomatic types for each ecosystem.

```
┌──────────────────────────────────────────────────────────┐
│                       User Code                          │
│         Python  /  Node.js  /  CLI  /  TOML config       │
└──────────────┬───────────────────────┬───────────────────┘
               │                       │
┌──────────────▼────────┐  ┌───────────▼───────────────────┐
│   Python surface      │  │        Node.js surface        │
│  python/asynmov/      │  │          npm/                 │
│  World · Corpus · CLI │  │  index.js · index.d.ts        │
└──────────────┬────────┘  └───────────┬───────────────────┘
               │ PyO3                  │ napi-rs
┌──────────────▼────────┐  ┌───────────▼───────────────────┐
│   crates/asynmov-py   │  │    crates/asynmov-node        │
│   (cdylib)            │  │    (cdylib)                   │
└──────────────┬────────┘  └───────────┬───────────────────┘
               └───────────┬───────────┘
                           │
              ┌────────────▼────────────┐
              │   crates/asynmov-core   │
              │  config · rng ·         │
              │  generators::entities   │
              └─────────────────────────┘
```

---

## Workspace layout

```
Cargo.toml                        # workspace root; [profile.release] here
crates/
  asynmov-core/                   # pure Rust, no FFI dependencies
    src/
      lib.rs
      config.rs                   # WorldConfig, WorldMeta, ConfigError
      rng.rs                      # make_seed(Option<u64>) → u64
      generators/
        mod.rs
        entities.rs               # AttrSpec, AttrDist, GeneratedData, Column
  asynmov-py/                     # PyO3 cdylib → asynmov._core
    src/lib.rs
  asynmov-node/                   # napi-rs cdylib
    src/lib.rs
    build.rs
npm/                              # Node package root
  index.js                        # platform-native loader
  index.d.ts                      # TypeScript types
  package.json                    # @asynmov/asynmov
  linux-x64-gnu/package.json      # @asynmov/asynmov-linux-x64-gnu
  linux-x64-musl/package.json
  linux-arm64-gnu/package.json
  darwin-x64/package.json
  darwin-arm64/package.json
  win32-x64-msvc/package.json
python/
  asynmov/
    __init__.py                   # re-exports: World, make_seed, validate_config, generate_from_toml
    world.py                      # World dataclass — thin handle over TOML src
    corpus.py                     # Corpus — holds polars.DataFrame, drives export
    cli.py                        # argparse CLI: generate / validate
    py.typed
```

---

## asynmov-core

The entire business logic lives here. There are no PyO3 or napi-rs imports.

### `config.rs`

Deserialises a TOML string into `WorldConfig` using the `toml` crate:

```rust
pub struct WorldConfig {
    pub world: WorldMeta,        // name, seed: Option<u64>
    pub attributes: Vec<AttrSpec>,
}
```

`WorldConfig::from_toml(src: &str) -> Result<Self, ConfigError>` is the only entry point for config parsing. Both language surfaces call this.

### `rng.rs`

```rust
pub fn make_seed(seed: Option<u64>) -> u64
```

Returns the seed unchanged if provided, or draws from OS entropy via `SmallRng::from_os_rng()`.

### `generators/entities.rs`

**Input:** `seed: u64`, `scale: u64`, `specs: &[AttrSpec]`

**Output:** `GeneratedData { ids: Vec<u64>, columns: Vec<(String, Column)> }`

`Column` is an enum:

```rust
pub enum Column {
    Int(Vec<i64>),
    Float(Vec<f64>),
    Bool(Vec<bool>),
    Utf8(Vec<String>),
}
```

The generator is **column-parallel**: Rayon maps over the spec slice, each column generated independently. Within a column, per-entity RNGs are derived deterministically from the root seed so results are identical regardless of thread scheduling:

```
entity_seed = root_seed.wrapping_add(id).wrapping_mul(6364136223846793005)
```

Each column's generator fast-forwards its RNG past prior columns to maintain independence without row-major allocation.

**Supported distributions** (via `AttrDist`):

| Variant | Column type |
|---------|-------------|
| `UniformInt { low, high }` | `Int` |
| `UniformFloat { low, high }` | `Float` |
| `Normal { mean, std_dev, min?, max? }` | `Float` |
| `Choice { values: Vec<ChoiceValue>, weights? }` | determined by first value: `Bool` / `Int` / `Float` / `Utf8` |

`ChoiceValue` is `#[serde(untagged)]` so TOML booleans, integers, floats, and strings all deserialise correctly.

---

## asynmov-py

A ~50-line cdylib. Exposes three functions to Python:

| Function | Returns |
|----------|---------|
| `make_seed(seed=None)` | `int` |
| `validate_config(toml_str)` | `None` or raises `ValueError` |
| `generate_from_toml(toml_str, scale)` | `polars.DataFrame` |

`generate_from_toml` calls `WorldConfig::from_toml`, then `rng::make_seed`, then `entities::generate`, and converts `GeneratedData.columns` into Polars `Series` via `pyo3-polars`. The DataFrame is returned zero-copy — no intermediate JSON.

Built with `pyo3 abi3-py311` so a single wheel runs on CPython 3.11, 3.12, and 3.13.

---

## asynmov-node

A ~60-line cdylib. Exposes three functions via napi-rs:

| Function | Returns |
|----------|---------|
| `makeSeed(seed?)` | `number` |
| `validateConfig(tomlStr)` | `void` or throws |
| `generateFromToml(tomlStr, scale)` | `string` (JSON array of row objects) |

`generateFromToml` calls the same core path as the Python surface, then transposes the columnar result back to row-major JSON for idiomatic JavaScript consumption.

---

## Python surface

`python/asynmov/` is purely a convenience layer — no logic:

- **`World`** — loads a TOML file, calls `validate_config`, holds `toml_src: str` for Rust to parse
- **`Corpus`** — wraps the `polars.DataFrame` returned by `generate_from_toml`; drives Polars export writers
- **`cli.py`** — argparse wrapper around `World.from_config` + `Corpus.export`

---

## Data flow

```
TOML file on disk
    │
    ▼ Path.read_text()            [Python]
    │
    ▼ validate_config(toml_src)   [Rust: toml::from_str → WorldConfig]
    │
    ▼ generate_from_toml(toml_src, scale)
          │
          ▼ WorldConfig::from_toml()     [Rust]
          ▼ rng::make_seed(cfg.world.seed)
          ▼ entities::generate(seed, scale, &cfg.attributes)
                → GeneratedData (columnar, parallel)
          ▼ columns → polars::Series → DataFrame
          │
    ▼ Corpus(df)                  [Python]
    │
    ▼ corpus.export(dir, format)  [Polars write_parquet / write_json / ...]
```

---

## Build system

| Surface | Tool | Config |
|---------|------|--------|
| Python wheel | maturin | `pyproject.toml` → `[tool.maturin]` |
| Node native | napi-rs CLI | `npm/package.json` → `napi` key |
| Rust workspace | cargo | `Cargo.toml` (workspace root) |

**Python dev build:**
```sh
maturin build --release --manifest-path crates/asynmov-py/Cargo.toml --out dist
pip install --no-index --find-links dist asynmov
```

**Node dev build:**
```sh
cd npm && npm run build:debug
```

CI builds for all platform targets are defined in `.github/workflows/` — `ci.yml` for tests, `publish.yml` for release.
