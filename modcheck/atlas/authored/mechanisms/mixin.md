# Mixin: composition and ordering, as extracted and as executed

Mixin (`sponge-mixin 0.17.4+mixin.0.8.7`, with MixinExtras 0.5.5 bundled by the
loader; both hashed in `extracted:corpus.json`) is the one mechanism by which
Fabric API itself, and any mod, changes vanilla bytecode. This note separates
three kinds of evidence and names each.

## Read from the jars (direct_reference)

- Annotation defaults, from the `AnnotationDefault` attributes of the pinned jar:
  `@Mixin.priority` 1000, `remap` true, `value`/`targets` empty; `@Inject.require` -1
  (meaning "use the config's default"), `expect` 1, `cancellable` false
  (`extracted:fabric_api.json`, `mixin_runtime.annotation_defaults`).
- Across the 47 modules, 512 mixin classes are declared in configs and 512 are
  found by annotation. Injector kinds actually used: `@Inject` 407, `@WrapOperation`
  113, `@Redirect` 58, `@ModifyExpressionValue` 44, `@ModifyArg` 32,
  `@ModifyReturnValue` 20, `@WrapMethod` 15, `@ModifyVariable` 13,
  `@WrapWithCondition` 6, `@ModifyReceiver` 1, and **one `@Overwrite`**
  (`fabric-renderer-api-v1`, `BlockStateModelWrapperMixin.update`, client). The
  earlier note said zero overwrites and no MixinExtras use; that came from a
  javap-text parser that could not see argument-less marker annotations or the
  `com.llamalad7` namespace. It was wrong on both counts and has been regenerated
  from the class files (`extracted:edges.json#replaces`, `extracted:edges.json#wraps`).
- Every injection target and every `@At` point is resolved against the processed
  compile jar: 126 exact by descriptor, 604 by unique name, 6 ambiguous among
  overloads, 5 wildcard/regex/quantified selectors; points 332 exact, 49 inherited,
  4 expression-based (question:q.selector_ambiguity).
- `InjectionPoint.checkPriority` returns true iff the target's priority is lower
  than the mixin's; `MethodHead`, `BeforeReturn` and `BeforeFinalReturn` override
  it to always true; `BeforeInvoke` and the other points do not
  (`extracted:mixin_transformation_tests.json`, `static_checks`).
- The applicator's pass order is MAIN, INJECT_PREPARE, ..., INJECT_APPLY: every
  mixin's methods (including overwrites) are merged before any mixin's injections
  are applied (`static_checks.ApplicatorPass.order`).

## Executed on controlled classes (executed_transformation)

The pinned transformer was run in a plain JVM over independently compiled target
classes and two mixin configurations standing in for two mods
(`atlas/harness/mixin_transform`, results in `extracted:mixin_transformation_tests.json`).
This is transformer evidence, not Minecraft evidence: no game class and no loader
classloader were involved.

| pair | result | scenario |
|---|---|---|
| `@Overwrite` + another mod's HEAD/TAIL `@Inject`, any priority | both apply; the injections run around the overwritten body | scenario:A, scenario:L, scenario:N |
| `@Overwrite` + another mod's INVOKE-point `@Inject` or `@Redirect`, equal or lower priority | refused: "cannot inject into ... merged by ... with priority"; with a required config, a transformation failure | scenario:B, scenario:B2, scenario:J |
| same, injecting mixin at higher priority, call still present | applies and fires | scenario:K |
| same, higher priority, call removed by the overwrite | fails the injection count check (require=1) | scenario:M |
| two `@Redirect`s of one call site | the second finds no target: failure with require=1 | scenario:C |
| `@Redirect` + `@WrapOperation` on one call site | compose; the wrapper runs around the redirect handler | scenario:D |
| two `@Overwrite`s at equal priority | no error; the first configuration's body runs | scenario:F |
| equal-priority HEAD injections from two configs | configuration order | scenario:E |
| priorities 900 and 1100 at HEAD | the lower priority's injection runs first | scenario:G |
| cancelling HEAD + another mod's TAIL/RETURN | the cancel suppresses them | scenario:H |

The previous version of this note asserted, without evidence, that "a mod's
`@Overwrite` of any injected method removes Fabric's injection and the events it
fires". That is false as a universal statement: HEAD/TAIL/RETURN injections
survive; INVOKE-point ones fail loudly rather than vanish; and the outcome
depends on priority. The claim is withdrawn and replaced by the table.

## What follows for a compatibility check (analyst inference)

1. A shared target is a **potential** interaction, nothing more
   (`50-Interactions/shared_targets`: 34 methods, all exactly resolved).
2. A conflict verdict needs, in order: exact resolution of both selectors and
   points; applicability (both mixins active in the same environment); and the
   composition rule for the pair of injector effects and priorities from the
   table above. Same-method overlap without same-point resolution is not a verdict
   (question:q.injection_points_not_resolved).
3. Nothing here is `observed` in a game (question:q.runtime_mixin_application).
