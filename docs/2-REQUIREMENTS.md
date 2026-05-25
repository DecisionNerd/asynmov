# Requirements

## Functional Requirements

### World Configuration

- **R-CFG-1**: The system must accept a world configuration file (TOML) that defines the parameters of the world to be generated.
- **R-CFG-2**: Configurations must be validated at load time with clear, actionable error messages for invalid values.
- **R-CFG-3**: Configurations must support a random seed for deterministic output.
- **R-CFG-4**: The system must ship built-in setting presets (e.g. `american_historical`, `fantasy_medieval`) that require minimal additional configuration.

### Entity Generation

- **R-ENT-1**: The system must generate character entities with configurable attribute sets (name, age, gender, occupation, background, etc.).
- **R-ENT-2**: Generated entities must be internally consistent — attributes must not contradict each other (e.g. age and era of birth must align).
- **R-ENT-3**: Entity counts must be configurable via a `scale` parameter.
- **R-ENT-4**: The system must support extension with user-defined entity types and attribute schemas.

### Event Generation

- **R-EVT-1**: The system must generate life events (birth, death, marriage, migration, occupation change, etc.) for each entity.
- **R-EVT-2**: Events must be causally ordered — an event cannot precede its prerequisites.
- **R-EVT-3**: Events must reference the correct entity IDs and timestamps.
- **R-EVT-4**: Event frequency and probability must be configurable per world setting.

### Relationship Generation

- **R-REL-1**: The system must generate relationships between entities (family, social, professional).
- **R-REL-2**: Relationships must be bidirectional and stored with metadata (type, strength, start/end date).
- **R-REL-3**: Social network density and clustering must be configurable.

### Output

- **R-OUT-1**: The system must export corpora in JSON format.
- **R-OUT-2**: The system must export corpora in CSV format (one file per entity/event/relationship type).
- **R-OUT-3**: The system must export corpora in JSONL format for streaming use cases.
- **R-OUT-4**: All output must reference a stable entity ID scheme so records can be joined across files.

### CLI

- **R-CLI-1**: A `asynmov generate` command must expose the full generation pipeline.
- **R-CLI-2**: A `asynmov validate` command must validate a config file without generating data.
- **R-CLI-3**: CLI flags must mirror the Python API parameters.

---

## Non-Functional Requirements

### Performance

- **R-PERF-1**: Generation of 10,000 entities with full event and relationship graphs must complete in under 60 seconds on a modern laptop (single core).
- **R-PERF-2**: Generation must scale near-linearly with entity count up to at least 1,000,000 entities when parallelism is available.
- **R-PERF-3**: Memory usage must not grow unboundedly with corpus size; the system must support streaming output for large corpora.

### Correctness

- **R-COR-1**: Identical seed + config must produce byte-identical output across platforms (Linux, macOS, Windows).
- **R-COR-2**: Generated data must satisfy all configured world constraints — no constraint violations in output.

### Usability

- **R-USE-1**: The package must install via `pip install asynmov` with no system-level build dependencies for end users (pre-built Rust wheels must be provided for common platforms).
- **R-USE-2**: Python 3.13+ is the minimum supported version.

### Maintainability

- **R-MNT-1**: The Rust/Python boundary must be isolated to a single integration layer; Python business logic must not be duplicated in Rust.
- **R-MNT-2**: All public APIs must be covered by tests before being marked stable.

---

## Out of Scope

- Real-time or interactive generation (web socket / streaming API).
- Hosting or serving generated corpora.
- Prose or natural language generation from structured data.
