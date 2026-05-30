from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path


@dataclass
class World:
    """Thin handle around a loaded TOML config."""

    name: str
    toml_src: str

    @classmethod
    def from_config(cls, path: str | Path) -> "World":
        src = Path(path).read_text(encoding="utf-8")
        from asynmov._core import validate_config
        validate_config(src)  # raises ValueError on bad config
        # extract name for display; Rust owns the real parse
        import tomllib
        meta = tomllib.loads(src).get("world", {})
        return cls(name=meta.get("name", "unnamed"), toml_src=src)

    def generate(self, scale: int = 100) -> "Corpus":
        from asynmov.corpus import Corpus
        return Corpus.generate(self, scale)
