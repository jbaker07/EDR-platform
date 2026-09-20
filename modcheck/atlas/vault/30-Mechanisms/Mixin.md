---
type: "mechanism"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# Mixin (direct bytecode modification)

Fabric API's own mixins in the corpus: 544 `injects_into`, 197 `wraps` and 1 `replaces` edges, by injector: @Inject 421, @WrapOperation 115, @Redirect 60, @ModifyExpressionValue 45, @ModifyArg 32, @ModifyVariable 26, @ModifyReturnValue 20, @WrapMethod 15, @WrapWithCondition 6, @ModifyReceiver 1, @Overwrite 1. A mod may use the same mechanism.

Resolution of the selectors against the processed jar: {"exact": 93, "name_only": 441, "ambiguous": 5, "selector_unsupported": 5} (injects_into), {"exact": 33, "name_only": 163, "ambiguous": 1} (wraps); points: {"exact": 332, "inherited_exact": 49, "selector_unsupported": 4}.

Runtime: `sponge-mixin` and `mixinextras` -- see [[00-Scope/Corpus]]. Annotation defaults read from the jar: `{"value": [], "targets": [], "priority": 1000, "remap": true}`.

How injectors compose is not asserted here; it was executed: [[30-Mechanisms/Transformation_Tests]].

Analyst note: [[_authored/mechanisms/mixin|composition and ordering]]
