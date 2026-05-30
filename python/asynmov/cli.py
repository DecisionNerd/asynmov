from __future__ import annotations

import argparse
import sys
from pathlib import Path


def main() -> None:
    parser = argparse.ArgumentParser(
        prog="asynmov",
        description="Generate large corpora of synthetic data for world building.",
    )
    sub = parser.add_subparsers(dest="command", required=True)

    gen = sub.add_parser("generate", help="Generate a world corpus.")
    gen.add_argument("--config", required=True, metavar="FILE", help="Path to world TOML config.")
    gen.add_argument("--scale", type=int, default=100, metavar="N", help="Number of entities to generate.")
    gen.add_argument("--output", required=True, metavar="DIR", help="Output directory.")
    gen.add_argument(
        "--format",
        choices=["parquet", "json", "csv", "jsonl"],
        default="parquet",
        help="Output format (default: parquet).",
    )

    val = sub.add_parser("validate", help="Validate a config file without generating.")
    val.add_argument("config", metavar="FILE", help="Path to world TOML config.")

    args = parser.parse_args()

    if args.command == "generate":
        _cmd_generate(args)
    elif args.command == "validate":
        _cmd_validate(args)


def _cmd_generate(args: argparse.Namespace) -> None:
    from asynmov.world import World

    world = World.from_config(args.config)
    corpus = world.generate(scale=args.scale)
    corpus.export(args.output, format=args.format)
    n = len(corpus.entities) if corpus.entities is not None else 0
    print(f"Generated {n} entities → {args.output}")


def _cmd_validate(args: argparse.Namespace) -> None:
    from asynmov.world import World

    try:
        World.from_config(args.config)
        print(f"Config OK: {args.config}")
    except Exception as e:
        print(f"Config invalid: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
