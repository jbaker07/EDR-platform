---
type: "workflow"
id: "wf.behaviour.rules_config"
area: "behaviour"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Make behaviour configurable (game rules, config)

**Intent.** Server operators should be able to switch or tune the mod's behaviour per world without editing files, and the setting should persist with the world.

## Must be preserved

- Vanilla game rules and their defaults.

## Mechanisms that can serve it

- [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] -- registers a game rule (`capability/add_configuration.fabric_gamerule`); read through [[40-Interfaces/net.minecraft.server.level.ServerLevel|ServerLevel]] getGameRules().
- A config file read at initialisation (no Fabric API module; loader gives the config directory through [[40-Interfaces/net.fabricmc.loader.api.FabricLoader|FabricLoader]]).

## Tools and artifacts used today

- The generated gamerule code; /gamerule in-game.

## Decisions the creator must make

- Per-world game rule (operator-facing, synced, persisted by vanilla) versus config file (per installation, needs own sync if the client must know).

## Information those decisions need

- That getGameRules() lives on ServerLevel, not Level (corrected after a build failure; `capability/add_configuration.fabric_gamerule`).

## Existing automation

- Deterministic gamerule generator in ModCheck.

## Remaining manual or unsupported work

- Config-file path.

## ModCheck's contribution

- Already delivers the game rule path; a config-file capability is a candidate.

## Evidence

- `capability/add_configuration.fabric_gamerule`
- `extracted/minecraft_surface.json.gz`

## Status

- inventoried: True
- mechanically_inspected: True
- contract_mapped: True
- interaction_analysed: True
- implemented_in_modcheck: True
- validated_scope: the gamerule generator compiles against the pinned corpus; the reference lantern registers its rule by hand with GameRuleBuilder and unit-tests the bounds (3 tests); no game run
