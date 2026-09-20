"""Which creator capabilities each game pack can actually answer.

The point of this map is the gaps. A blank cell is the useful output: it says
we have no recorded mechanism for that capability in that game, which is a
bounded, nameable piece of work -- not a vague sense that coverage is uneven.

Nothing here upgrades a claim. A cell is filled only where a capability record
exists, and it carries that record's evidence state unchanged: a capability
derived from a documented recipe reads `documented`, and only a build that ran
here reads `build_tested`. There is no cell value meaning "probably fine".
"""
from __future__ import annotations

import dataclasses

from ..store import Store
from .capability import CREATOR_CAPABILITIES, IDS

# Ordered weakest to strongest. Used only to pick which record represents a
# cell when several exist -- never to average, score, or roll up.
EVIDENCE_ORDER = ("documented", "source_confirmed", "example_confirmed",
                  "build_tested", "game_tested")


@dataclasses.dataclass
class Cell:
    game: str
    capability: str
    records: list[str] = dataclasses.field(default_factory=list)
    evidence_state: str | None = None
    recipes: list[str] = dataclasses.field(default_factory=list)

    @property
    def covered(self) -> bool:
        return bool(self.records)


@dataclasses.dataclass
class CoverageMap:
    cells: dict[tuple[str, str], Cell]
    games: list[str]
    capabilities: list[str]

    def cell(self, game: str, capability: str) -> Cell:
        return self.cells.get((game, capability), Cell(game, capability))

    def gaps(self) -> list[tuple[str, str]]:
        return [(g, c) for g in self.games for c in self.capabilities
                if not self.cell(g, c).covered]

    def by_game(self, game: str) -> list[Cell]:
        return [self.cell(game, c) for c in self.capabilities]

    def summary(self) -> dict[str, int]:
        covered = sum(1 for g in self.games for c in self.capabilities
                      if self.cell(g, c).covered)
        total = len(self.games) * len(self.capabilities)
        states: dict[str, int] = {}
        for cell in self.cells.values():
            if cell.evidence_state:
                states[cell.evidence_state] = states.get(cell.evidence_state, 0) + 1
        return {"cells": total, "covered": covered, "gaps": total - covered} | states


def build_map(store: Store | None = None) -> CoverageMap:
    store = store or Store()
    capabilities = [c.id for c in CREATOR_CAPABILITIES]
    cells: dict[tuple[str, str], Cell] = {}
    for game in store.games:
        for record in store.pack(game).records("capability"):
            cap = record.get("capability")
            if cap not in IDS:
                # An unknown id is a record pointing at nothing. Skipped here and
                # reported by `modcheck validate`, rather than quietly widening
                # the taxonomy to whatever a file happened to say.
                continue
            cell = cells.setdefault((game, cap), Cell(game, cap))
            cell.records.append(record.id)
            cell.recipes += [r for r in (record.get("recipes") or [])
                             if r not in cell.recipes]
            state = record.get("evidence_state")
            if state in EVIDENCE_ORDER:
                current = cell.evidence_state
                if current is None or EVIDENCE_ORDER.index(state) > EVIDENCE_ORDER.index(current):
                    cell.evidence_state = state
    return CoverageMap(cells=cells, games=list(store.games), capabilities=capabilities)


_MARK = {"documented": "doc", "source_confirmed": "src", "example_confirmed": "ex",
         "build_tested": "BLD", "game_tested": "GAME"}


def render(coverage: CoverageMap) -> str:
    width = max(len(c) for c in coverage.capabilities) + 2
    lines = ["Creator capability coverage", ""]
    header = " " * width + "".join(f"{g[:6]:>7}" for g in coverage.games)
    lines.append(header)
    for capability in coverage.capabilities:
        row = f"  {capability:<{width - 2}}"
        for game in coverage.games:
            cell = coverage.cell(game, capability)
            row += f"{(_MARK.get(cell.evidence_state or '', '') or '-'):>7}"
        lines.append(row)
    lines.append("")
    summary = coverage.summary()
    lines.append(f"  {summary['covered']}/{summary['cells']} cells have a recorded "
                 f"mechanism; {summary['gaps']} do not.")
    lines.append("")
    lines.append("  doc = derived from a documented recipe   src = framework source read")
    lines.append("  ex  = working example read               BLD = built here")
    lines.append("  GAME = observed in a running game        -   = no record at all")
    lines.append("")
    lines.append("  A '-' is not 'unsupported'. It means we have recorded no mechanism,")
    lines.append("  which is a piece of work to do, not a property of the game.")
    lines.append("  A 'doc' cell rests on documentation alone: intended usage, not")
    lines.append("  verified behaviour.")
    return "\n".join(lines)
