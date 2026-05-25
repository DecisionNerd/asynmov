# Mission

## Problem

World building requires enormous amounts of coherent, internally consistent data. A novelist, game designer, or simulation researcher who wants a believable world needs hundreds or thousands of characters with plausible histories, interconnected relationships, locations with consistent geography, and timelines where cause precedes effect. Generating this by hand is prohibitively slow. Existing synthetic data tools are designed for database testing, not narrative coherence — they produce statistically valid rows, not meaningful stories.

## Vision

Asynmov generates large corpora of synthetic world-building data that are:

- **Coherent** — entities, events, and relationships are internally consistent with each other and with the world's rules.
- **Scalable** — a single invocation can produce corpora of tens of thousands of interconnected records without hitting performance walls.
- **Configurable** — the world's parameters (era, geography, culture, technology level, etc.) drive all generated content.
- **Composable** — outputs are structured data (JSON, CSV, and others) that plug into downstream tools: game engines, LLM fine-tuning pipelines, narrative databases, simulation frameworks.

## Target Users

| User | Need |
|---|---|
| Fiction authors | Character histories, event timelines, and social networks for a setting |
| Game designers | NPC populations, lore corpora, and faction relationship graphs |
| Simulation researchers | Synthetic populations with realistic demographic and behavioural attributes |
| ML / LLM practitioners | Large-scale narrative training data with ground-truth structure |

## Goals

1. Make it trivial to generate a coherent world corpus from a minimal configuration.
2. Sustain throughput sufficient for corpora in the millions of records via the Rust backend.
3. Keep the Python API the single integration surface — users never need to know Rust exists.
4. Remain setting-agnostic at the core; settings (historical, fantasy, sci-fi, etc.) are configurations, not forks.

## Non-Goals

- Asynmov does not generate prose narrative. It generates structured data that prose can be derived from.
- Asynmov does not validate historical accuracy of user-supplied configurations. Garbage in, plausible garbage out.
- Asynmov is not a game engine or simulation runtime. It produces static corpora, not live simulations.
