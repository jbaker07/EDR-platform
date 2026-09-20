---
type: "mechanism"
module: "fabric-rendering-fluids-v1"
version: "6.0.6+3434d6d95d"
sha256: "ce4c943f676d105275247800ce1a7ed4a6410c95b3b98ef1b39aba4dcf8a4264"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-rendering-fluids-v1

**Version** `6.0.6+3434d6d95d` -- **artifact sha256** `ce4c943f676d105275247800ce1a7ed4a6410c95b3b98ef1b39aba4dcf8a4264`

## Declared (fabric.mod.json)

- environment: `*`
- depends: `{"fabricloader": ">=0.19.3", "fabric-api-base": "*"}`
- entrypoints: `null`
- mixin configs: `[{"config": "fabric-rendering-fluids-v1.mixins.json", "environment": "client"}]`
- access widener: `fabric-rendering-fluids-v1.classtweaker`

## Events this module publishes

- none found by extraction

## Vanilla types this module modifies (mixins)

| vanilla type | method | how | environment | mixin |
|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.renderer.block.FluidRenderer|FluidRenderer]] | `tesselate` | injects_into `@Inject at HEAD` | client | `FluidRendererMixin.onHeadRender` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.client.render.fluid.v1.FluidRenderHandler|FluidRenderHandler]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.render.fluid.v1.FluidRendering|FluidRendering]] (class, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.render.fluid.v1.FluidRenderingRegistry|FluidRenderingRegistry]] (class, 8 members)

## What this establishes, and does not

- Injection targets and API signatures are `direct_reference`: read from the jar.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- Nothing here is `observed`. No game ran.
