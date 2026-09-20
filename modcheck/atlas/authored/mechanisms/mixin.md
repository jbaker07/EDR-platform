# Mixin: composition, ordering, and what the corpus actually shows

Mixin (`sponge-mixin 0.17.4+mixin.0.8.7`, in `extracted:corpus.json`) is the one
mechanism by which Fabric API itself, and any mod, changes vanilla bytecode. The
generated `mechanism:Mixin` note carries the counts; this note is the analyst's
reading of what those counts mean for composition.

## What was read from the jars (direct_reference)

- `@Mixin.priority()` has annotation default **1000**, `@Mixin.remap()` defaults
  to true, `@Mixin.targets()` defaults to empty. Read with
  `javap -v org.spongepowered.asm.mixin.Mixin` against the sponge-mixin artifact
  in the corpus.
- `@Inject` declares `cancellable()`, `require()`, `expect()` and `order()`
  members (same method, `org.spongepowered.asm.mixin.injection.Inject`).
- Across all 47 Fabric API modules the injector kinds actually used are
  `@Inject` 407, `@Redirect` 58, `@ModifyArg` 32, `@ModifyVariable` 13, and
  **zero `@Overwrite`** (`extracted:fabric_api.json`; edges in
  `extracted:edges.json#injects_into` and `extracted:edges.json#wraps`).
- Every mixin config declares its `package`, `mixins`, `client` and `server`
  lists; the environment of each edge in the atlas comes from those lists or from
  an `@Environment` annotation on the mixin class (`extracted:fabric_api.json`).

## What follows from that (analyst inference)

1. **Additive injections coexist; redirects contend.** An `@Inject` adds a call
   at a location and, unless `cancellable` and cancelled, leaves the original flow
   intact, so many mods injecting into one method is the normal case. A
   `@Redirect` replaces one call site; two redirects of the same call site cannot
   both apply. The generated `50-Interactions/contested_methods` list is where a
   mod's own redirect or overwrite would collide with Fabric API.
2. **`@Overwrite` is avoided by Fabric API itself.** Zero overwrites in 47
   modules is a design signal: Fabric API composes with other mods by never
   replacing a vanilla method body. A mod that overwrites a method Fabric API
   injects into removes Fabric's injection, and with it every event fired from it.
3. **Priority is a tie-break, not an ordering guarantee.** The default 1000 is
   read from the jar. How the runtime orders mixins of equal priority across mods
   is documented upstream but **not in the corpus**
   (question:q.mixin_docs_application_order) and not observed
   (question:q.runtime_mixin_application). Nothing in this atlas states the
   application order of two mods' mixins.
4. **Cancellation is a contract on the injection, not on the event.** A
   cancellable `@Inject` lets the handler skip the rest of the vanilla method.
   Whether a Fabric *event* lets a subscriber cancel is a property of the
   callback's return type (`declared`), recorded per event in `50-Interactions/events`.

## What the extraction does not resolve

- `@At` injection points are recorded as strings (`INVOKE`, `TAIL`, `HEAD`,
  `RETURN`, `NEW`, with their `target` descriptors) and are not resolved to
  bytecode offsets, slices or ordinals (question:q.injection_points_not_resolved).
- Whether an injection's target method exists in 26.3 is cross-checked by
  `extracted:minecraft_members.json` for the hooked types; the edge targets that remain
  unresolved are listed there with the reasons (question:q.edge_targets_unresolved).
- ModCheck's own jar inspector lists a mod's mixin classes but does not read their
  targets, so the collision analysis this note describes is **not implemented**
  for arbitrary mod jars (question:q.inspector_mixin_targets,
  question:q.mixin_collision_analyser).
