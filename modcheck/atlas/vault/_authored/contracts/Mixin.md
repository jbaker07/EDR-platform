---
type: "contract"
subject: "mechanism:Mixin"
kind: "hook"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# Contract: Mixin

Subject: [[30-Mechanisms/Mixin|Mixin]]

## Asserted

| claim | basis | evidence |
|---|---|---|
| Mixin priority defaults to 1000 and the target list to empty (annotation defaults read from the pinned jar). | `direct_reference` | `extracted/fabric_api.json`; [[30-Mechanisms/Mixin|Mixin]] |
| Fabric API 0.161.0+26.3 carries exactly one @Overwrite (fabric-renderer-api-v1 BlockStateModelWrapperMixin.update, client) and 199 MixinExtras injections; the earlier 'zero overwrites' figure came from a parser that could not see marker annotations. | `direct_reference` | `extracted/edges.json#replaces`; `extracted/fabric_api.json` |
| An @Overwrite does NOT remove another mixin's HEAD, TAIL or RETURN injections into the same method: they apply to the overwritten body, at any priority ([[30-Mechanisms/Transformation_Tests#A|scenario A]], [[30-Mechanisms/Transformation_Tests#L|scenario L]], [[30-Mechanisms/Transformation_Tests#N|scenario N]]). | `executed_transformation` | `extracted/mixin_transformation_tests.json`; [[30-Mechanisms/Transformation_Tests#A|scenario A]]; [[30-Mechanisms/Transformation_Tests#L|scenario L]]; [[30-Mechanisms/Transformation_Tests#N|scenario N]] |
| An INVOKE-point @Inject or a @Redirect into a method overwritten by a mixin of equal or higher priority is refused at transformation time ('cannot inject into ... merged by ... with priority'), which with a required config is a startup failure, not a silent removal ([[30-Mechanisms/Transformation_Tests#B|scenario B]], [[30-Mechanisms/Transformation_Tests#B2|scenario B2]], [[30-Mechanisms/Transformation_Tests#J|scenario J]]). The rule is InjectionPoint.checkPriority: MethodHead, BeforeReturn and BeforeFinalReturn override it to true; BeforeInvoke and the other points do not. | `executed_transformation` | `extracted/mixin_transformation_tests.json`; [[30-Mechanisms/Transformation_Tests#B|scenario B]]; [[30-Mechanisms/Transformation_Tests#B2|scenario B2]]; [[30-Mechanisms/Transformation_Tests#J|scenario J]] |
| With a HIGHER priority than the overwrite, an INVOKE-point injection applies to the overwritten body and fires if the body still contains the call ([[30-Mechanisms/Transformation_Tests#K|scenario K]]); if the overwritten body no longer contains the call it fails the injection count check with require=1 ([[30-Mechanisms/Transformation_Tests#M|scenario M]]). | `executed_transformation` | [[30-Mechanisms/Transformation_Tests#K|scenario K]]; [[30-Mechanisms/Transformation_Tests#M|scenario M]] |
| Two @Redirects of one call site from two mods do not both apply: the second finds no target and fails with require=1 ([[30-Mechanisms/Transformation_Tests#C|scenario C]]). A MixinExtras @WrapOperation over a @Redirect composes: the wrapper runs around the redirect handler ([[30-Mechanisms/Transformation_Tests#D|scenario D]]). | `executed_transformation` | [[30-Mechanisms/Transformation_Tests#C|scenario C]]; [[30-Mechanisms/Transformation_Tests#D|scenario D]] |
| Two @Overwrites of one method at equal priority: no error; the first configuration's body is what runs ([[30-Mechanisms/Transformation_Tests#F|scenario F]]). Lower priority is applied first and its HEAD injection runs first ([[30-Mechanisms/Transformation_Tests#G|scenario G]]); at equal priority, configuration order decides ([[30-Mechanisms/Transformation_Tests#E|scenario E]]). | `executed_transformation` | [[30-Mechanisms/Transformation_Tests#F|scenario F]]; [[30-Mechanisms/Transformation_Tests#G|scenario G]]; [[30-Mechanisms/Transformation_Tests#E|scenario E]] |
| A cancelling HEAD injection suppresses every later injection in that method, including another mod's TAIL and RETURN ([[30-Mechanisms/Transformation_Tests#H|scenario H]]). | `executed_transformation` | [[30-Mechanisms/Transformation_Tests#H|scenario H]] |

## Not established

- That any of this holds inside a running Minecraft with fabric-loader's Knot classloader: the tests ran the transformer alone ([[80-Unresolved/q.runtime_mixin_application|q.runtime_mixin_application]]).
- The documented rule for equal-priority ordering across mods ([[80-Unresolved/q.mixin_docs_application_order|q.mixin_docs_application_order]]).
- Same-point versus same-method overlap for the 34 shared targets ([[80-Unresolved/q.injection_points_not_resolved|q.injection_points_not_resolved]]).

## Evidence

- `extracted/mixin_transformation_tests.json`
- `extracted/edges.json#shared_targets`
- `extracted/fabric_api.json`

## Open questions

- [[80-Unresolved/q.mixin_docs_application_order|q.mixin_docs_application_order]]
- [[80-Unresolved/q.runtime_mixin_application|q.runtime_mixin_application]]
- [[80-Unresolved/q.injection_points_not_resolved|q.injection_points_not_resolved]]
