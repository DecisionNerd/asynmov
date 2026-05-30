from __future__ import annotations

from pathlib import Path
from typing import TYPE_CHECKING, Literal

if TYPE_CHECKING:
    import polars as pl
    from asynmov.world import World


class Corpus:
    """Holds generated entities as a Polars DataFrame."""

    def __init__(self, entities: "pl.DataFrame") -> None:
        self.entities = entities

    @classmethod
    def generate(cls, world: "World", scale: int) -> "Corpus":
        from asynmov._core import generate_from_toml
        return cls(generate_from_toml(world.toml_src, scale))

    def export(
        self,
        directory: str | Path,
        format: Literal["parquet", "json", "csv", "jsonl"] = "parquet",
    ) -> None:
        out = Path(directory)
        out.mkdir(parents=True, exist_ok=True)
        match format:
            case "parquet": self.entities.write_parquet(out / "entities.parquet")
            case "json":    self.entities.write_json(out / "entities.json")
            case "csv":     self.entities.write_csv(out / "entities.csv")
            case "jsonl":   self.entities.write_ndjson(out / "entities.jsonl")
            case _:         raise ValueError(f"Unknown format: {format!r}")
