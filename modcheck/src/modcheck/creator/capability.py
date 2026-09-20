"""The shared creator-capability taxonomy.

ModCheck is one creation workflow with game-specific knowledge, not ten
products. The seam between the two is here: a small set of organizing
categories that describe *what a creator is trying to do*, which every game
pack then answers in its own terms.

NOT to be confused with ``modcheck.paths.CAPABILITIES``, which is a different
list with the same word: those are MODCHECK's capabilities per game (can we
inspect a package, can we execute a build), declared in each ``pack.yaml`` and
graded supported/partial/blocked. These are the CREATOR's capabilities -- what
someone can do inside the game. The two lists never merge; a game pack can
declare ``build_execution: supported`` for us while a creator capability in
that same game is entirely undocumented.

These are categories for organizing evidence. They are deliberately NOT a
universal API. `register_content` means something different in a Fabric
registry, a RimWorld XML Def, a Sims 4 DBPF resource and a Content Patcher
EditData patch, and nothing here pretends otherwise -- each game's capability
record carries that game's exact mechanism, signature and preconditions
underneath the shared label.

The label is for navigation. The mechanism underneath it is the evidence.
"""
from __future__ import annotations

import dataclasses


@dataclasses.dataclass(frozen=True)
class Capability:
    id: str
    title: str
    description: str
    # What a creator typically says when they want this. Used for retrieval, not
    # as proof: a phrase match suggests a capability, it does not establish that
    # the game supports it.
    phrases: tuple[str, ...]


CREATOR_CAPABILITIES: tuple[Capability, ...] = (
    Capability(
        "register_content", "Register an item, object, spell, entity or data record",
        "Introduce a new named thing the game knows about, through whatever "
        "registry, definition file or data record the game uses for it.",
        ("add", "new", "item", "block", "entity", "spell", "weapon", "object",
         "record", "def", "register", "create", "recipe")),
    Capability(
        "subscribe_event", "Subscribe to an event",
        "Run code when the game reaches a defined point, through a documented "
        "callback, listener or hook.",
        ("when", "on", "event", "listener", "callback", "hook", "trigger",
         "subscribe", "tick", "fires")),
    Capability(
        "modify_behaviour", "Change a scoped part of existing behaviour",
        "Alter part of what the game already does, without replacing the whole "
        "system -- a mixin, a patch operation, a method wrap, a record override.",
        ("change", "modify", "patch", "override", "adjust", "extend", "alter",
         "mixin", "wrap", "behaviour", "behavior", "tweak")),
    Capability(
        "persist_state", "Store and restore custom state",
        "Keep data across a save and reload, in whatever the game treats as "
        "durable storage for mod data.",
        ("save", "persist", "store", "remember", "reload", "restore", "state",
         "keep", "across", "retain", "nbt")),
    Capability(
        "display_information", "Display information to the player",
        "Put something on screen -- a HUD element, tooltip, menu entry or "
        "message.",
        ("show", "display", "hud", "tooltip", "screen", "ui", "message",
         "render", "overlay", "indicator", "bar", "remaining")),
    Capability(
        "sync_state", "Synchronize client/server state",
        "Move data between the client and the server, respecting whichever side "
        "owns the authoritative value.",
        ("sync", "network", "packet", "client", "server", "multiplayer",
         "replicate", "authority")),
    Capability(
        "add_configuration", "Add configuration",
        "Let a player or server operator change the mod's behaviour without "
        "editing code.",
        ("config", "configure", "option", "setting", "setup", "toggle",
         "configurable", "adjustable", "sandbox")),
    Capability(
        "integrate_optional_dependency", "Integrate an optional dependency",
        "Use another mod's functionality when it is present, and degrade "
        "cleanly when it is not.",
        ("optional", "dependency", "integration", "soft", "compat",
         "if installed", "another mod")),
    Capability(
        "compose_asset", "Modify or compose an asset",
        "Change a texture, model, map or other asset, by replacement or by "
        "composition with what is already there.",
        ("texture", "sprite", "image", "model", "asset", "map", "overlay",
         "replace", "recolour", "recolor", "art")),
    Capability(
        "build_and_test", "Build and test a release",
        "Turn the project into a distributable artifact and check it.",
        ("build", "compile", "package", "release", "test", "jar", "pack",
         "publish", "artifact")),
)

BY_ID: dict[str, Capability] = {c.id: c for c in CREATOR_CAPABILITIES}
IDS: frozenset[str] = frozenset(BY_ID)


def _matches(phrase: str, words: set[str], lowered: str) -> bool:
    """Whether a request mentions a phrase.

    Multi-word phrases are matched against the whole text. Single words match on
    a shared prefix of at least four characters, so "stored" finds "store" and
    "configuration" finds "config" -- creators do not write in the stems a
    keyword list happens to use. Four is short enough to catch inflections and
    long enough that "con" does not match everything.
    """
    if " " in phrase:
        return phrase in lowered
    if phrase in words:
        return True
    stem = phrase[:4]
    if len(phrase) < 4:
        return False
    return any(word.startswith(stem) and (word.startswith(phrase)
                                          or phrase.startswith(word[:4]))
               for word in words if len(word) >= 4)


def suggest(text: str) -> list[tuple[str, list[str]]]:
    """Capabilities a request's wording suggests, with the words that suggested them.

    A lead, never a conclusion. The caller must still establish that the game
    in question actually supports the capability, which is what the capability
    records are for. Nothing downstream may treat a suggestion as evidence.
    """
    lowered = text.lower()
    words = {w.strip(".,;:!?\"'()[]").lower() for w in lowered.split()}
    hits = []
    for capability in CREATOR_CAPABILITIES:
        matched = sorted(p for p in capability.phrases if _matches(p, words, lowered))
        if matched:
            hits.append((capability.id, matched))
    hits.sort(key=lambda pair: (-len(pair[1]), pair[0]))
    return hits
