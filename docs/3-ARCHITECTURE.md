# Architecture

## Overview

Asynmov is a two-layer system: a **Python package** that owns the public API, configuration, and orchestration, and a **Rust core** that owns high-throughput data generation. The layers communicate through a thin PyO3 binding layer.

```
┌─────────────────────────────────────────────┐
│                  User Code                  │
│         Python  /  CLI  /  Config           │
└─────────────────────┬───────────────────────┘
                      │
┌─────────────────────▼───────────────────────┐
│              Python Package                 │
│   World  │  Config  │  Corpus  │  Exporters │
└─────────────────────┬───────────────────────┘
                      │  PyO3 bindings
┌─────────────────────▼───────────────────────┐
│                 Rust Core                   │
│  Generators  │  Constraint Engine  │  RNG   │
└─────────────────────────────────────────────┘
```

---

## Python Layer

The Python layer is responsible for:

- **Configuration** — loading, validating, and normalising TOML/dict world configs.
- **Orchestration** — driving the generation pipeline, managing entity IDs, sequencing generation passes.
- **Corpus management** — collecting and indexing generated records in memory or on disk.
- **Export** — serialising corpora to JSON, CSV, JSONL.
- **CLI** — the `asynmov` command-line tool.
- **Extension API** — hooks that users can implement to add custom generators or constraints.

### Package layout (intended)

```
asynmov/
  __init__.py          # public API re-exports
  world.py             # World class — top-level entry point
  config.py            # Config loading and validation
  corpus.py            # Corpus collection and indexing
  exporters/
    json.py
    csv.py
    jsonl.py
  generators/          # Python-side generator interfaces (delegate to Rust)
    characters.py
    events.py
    relationships.py
  _core/               # PyO3 extension module (compiled Rust)
```

---

## Rust Core

The Rust core is responsible for:

- **High-throughput generation** — producing raw attribute values, names, dates, and identifiers at scale.
- **Constraint satisfaction** — enforcing world rules and causal ordering within and across entities.
- **Seeded RNG** — providing a deterministic, seed-driven random number source (e.g. `rand::rngs::SmallRng`).
- **Parallelism** — using Rayon for data-parallel generation across entity batches.

The Rust crate is compiled as a Python extension module using [PyO3](https://pyo3.rs/) and [maturin](https://github.com/PyO3/maturin). It exposes a minimal set of classes and functions to the Python layer — it does not implement business logic or know about configuration files.

### Crate layout (intended)

```
src/
  lib.rs               # PyO3 module definition
  rng.rs               # Seeded RNG wrappers
  generators/
    names.rs
    dates.rs
    attributes.rs
    events.rs
    relationships.rs
  constraints/
    engine.rs
    rules.rs
```

---

## Data Flow

```
Config file
    │
    ▼
Config::load()           # Python: validate and normalise
    │
    ▼
World::generate(scale)   # Python: orchestrate passes
    │
    ├─► Pass 1: generate entity skeletons (Rust)
    │       → list of {id, seed_attributes}
    │
    ├─► Pass 2: resolve entity attributes (Rust + constraints)
    │       → list of {id, full_attributes}
    │
    ├─► Pass 3: generate events per entity (Rust)
    │       → list of {entity_id, event_type, timestamp, payload}
    │
    ├─► Pass 4: generate relationships (Rust)
    │       → list of {entity_a, entity_b, type, metadata}
    │
    └─► Corpus::collect()  # Python: assemble and index
            │
            ▼
        Exporter::write()  # Python: serialise to disk
```

---

## Key Design Decisions

### Why Rust for the core?

Generating a corpus of 1M entities with events and relationships involves hundreds of millions of small computations. Python's GIL and interpreter overhead make this impractical at that scale. Rust provides C-level throughput and native parallelism without a separate process boundary.

### Why PyO3 over subprocess or gRPC?

PyO3 gives zero-copy data sharing between Python and Rust within the same process. There is no serialisation overhead at the boundary, and users install a single wheel — no daemon, no socket, no extra process.

### Why multi-pass generation?

Entities, events, and relationships have dependencies on each other. A single-pass approach would require speculative generation and rollback. Multi-pass makes dependencies explicit and enables parallel processing within each pass.

### Why TOML for configuration?

TOML is human-readable, has first-class support for tables and arrays, and maps cleanly to Python dicts and Rust structs. It is a better fit for hierarchical world configs than INI or flat JSON.

---

## Build System

Rust compilation is managed by [maturin](https://github.com/PyO3/maturin). For development:

```sh
pip install maturin
maturin develop          # compile Rust and install into current venv
```

For release, maturin builds platform wheels (Linux x86_64, Linux aarch64, macOS arm64, macOS x86_64, Windows x86_64) via CI.

Python packaging is managed via `pyproject.toml` with `maturin` as the build backend.
