from __future__ import annotations

from dataclasses import dataclass, field
from pathlib import Path
from typing import Any

from asynmov._core import make_seed


@dataclass
class World:
    """Top-level entry point for generating a world corpus."""

    name: str
    seed: int
    config: dict[str, Any] = field(default_factory=dict)

    @classmethod
    def from_config(cls, path: str | Path) -> "World":
        """Load a world from a TOML config file."""
        try:
            import tomllib
        except ImportError:
            raise RuntimeError("Python 3.11+ required for tomllib") from None

        with open(path, "rb") as f:
            raw = tomllib.load(f)

        world_cfg = raw.get("world", {})
        return cls(
            name=world_cfg.get("name", "unnamed"),
            seed=make_seed(world_cfg.get("seed")),
            config=raw,
        )

    def generate(self, scale: int = 100) -> "Corpus":
        """Generate a corpus of the given scale (number of entities)."""
        from asynmov.corpus import Corpus

        return Corpus.generate(world=self, scale=scale)
