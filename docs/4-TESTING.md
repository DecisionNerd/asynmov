# Testing

## Philosophy

Tests verify two things: that the system generates valid, constraint-satisfying data, and that the Python/Rust boundary behaves correctly. Performance benchmarks are tracked separately from correctness tests.

---

## Test Layers

### Rust unit tests

The Rust core has its own unit tests co-located with each module (`#[cfg(test)]` blocks). These test individual generators and the constraint engine in isolation, without crossing the Python boundary.

```
cargo test
```

What is tested:
- RNG determinism: same seed produces same sequence.
- Individual generators produce values within expected distributions.
- Constraint engine rejects invalid states and resolves valid ones.
- Causal ordering: events are always ordered correctly relative to prerequisites.

### Python unit tests

Python unit tests cover the configuration layer, corpus management, and exporters. They mock the Rust extension module where needed to test Python logic in isolation.

```
pytest tests/unit/
```

What is tested:
- Config validation: valid configs load, invalid configs raise with clear messages.
- Corpus indexing: entities and events are reachable by ID after collection.
- Exporter output: JSON, CSV, and JSONL outputs are well-formed and complete.
- CLI argument parsing: flags map to the correct API calls.

### Integration tests

Integration tests exercise the full Python → Rust → Python round-trip with a real compiled extension. They use a fixed seed and a small scale to keep runtime low.

```
pytest tests/integration/
```

What is tested:
- End-to-end generation produces a non-empty corpus with no constraint violations.
- Determinism: identical seed + config produces identical corpus across runs.
- All entity IDs referenced in events and relationships exist in the entity table.
- All output formats load back correctly (round-trip through exporter → parser).

### Property-based tests

Where output correctness is hard to specify exhaustively, property-based tests (via [Hypothesis](https://hypothesis.readthedocs.io/)) verify invariants across randomly generated inputs.

```
pytest tests/property/
```

Key properties tested:
- For any valid config, generation completes without error.
- For any generated corpus, referential integrity holds (no dangling IDs).
- For any corpus, entity count equals the requested scale (within any configured variance).

---

## Benchmarks

Benchmarks are not run in CI by default but live alongside the tests.

```
pytest tests/benchmarks/ --benchmark-only
```

Or for the Rust side:

```
cargo bench
```

Tracked metrics:
- Generation throughput: entities per second at scale 1k, 10k, 100k, 1M.
- Memory usage at each scale.
- Export throughput: records per second for each format.

---

## CI

All correctness tests (unit + integration + property) run on every pull request across:

- Linux x86_64 (primary)
- macOS arm64
- Windows x86_64

Rust tests run first (`cargo test`), then Python tests (`pytest`). A failing Rust test blocks the Python test run.

---

## What is not tested

- Historical accuracy of built-in setting presets — that is a content question, not a correctness question.
- Performance on machines outside the CI matrix.
- Behaviour with user-defined extension hooks beyond the documented API contract.

---

## Adding a test

1. For a new Rust generator: add a `#[test]` in the module's `#[cfg(test)]` block.
2. For a new Python API: add a unit test in `tests/unit/` and an integration test that exercises it end-to-end.
3. For a new invariant: add a Hypothesis strategy in `tests/property/` and document the property being checked.
