from __future__ import annotations

import json
import csv
from pathlib import Path
from typing import TYPE_CHECKING, Literal

if TYPE_CHECKING:
    from asynmov.world import World


class Corpus:
    """Holds the generated entities, events, and relationships for a world."""

    def __init__(self) -> None:
        self.entities: list[dict] = []
        self.events: list[dict] = []
        self.relationships: list[dict] = []

    @classmethod
    def generate(cls, world: "World", scale: int) -> "Corpus":
        from asynmov._core import generate_entities

        corpus = cls()

        attr_specs = world.config.get("attributes", [])
        if attr_specs:
            specs_json = json.dumps(attr_specs)
            corpus.entities = json.loads(generate_entities(world.seed, scale, specs_json))
        else:
            corpus.entities = [{"id": i, "world": world.name} for i in range(scale)]

        return corpus

    def export(
        self,
        directory: str | Path,
        format: Literal["json", "csv", "jsonl"] = "json",
    ) -> None:
        out = Path(directory)
        out.mkdir(parents=True, exist_ok=True)

        if format == "json":
            self._write_json(out)
        elif format == "csv":
            self._write_csv(out)
        elif format == "jsonl":
            self._write_jsonl(out)
        else:
            raise ValueError(f"Unknown format: {format!r}")

    def _write_json(self, directory: Path) -> None:
        (directory / "entities.json").write_text(json.dumps(self.entities, indent=2))
        (directory / "events.json").write_text(json.dumps(self.events, indent=2))
        (directory / "relationships.json").write_text(json.dumps(self.relationships, indent=2))

    def _write_jsonl(self, directory: Path) -> None:
        for name, records in [
            ("entities", self.entities),
            ("events", self.events),
            ("relationships", self.relationships),
        ]:
            with open(directory / f"{name}.jsonl", "w") as f:
                for record in records:
                    f.write(json.dumps(record) + "\n")

    def _write_csv(self, directory: Path) -> None:
        for name, records in [
            ("entities", self.entities),
            ("events", self.events),
            ("relationships", self.relationships),
        ]:
            if not records:
                continue
            with open(directory / f"{name}.csv", "w", newline="") as f:
                writer = csv.DictWriter(f, fieldnames=list(records[0].keys()))
                writer.writeheader()
                writer.writerows(records)
