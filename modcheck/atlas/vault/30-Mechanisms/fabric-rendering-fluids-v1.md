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
- mixin classes: 2 found by annotation, 2 declared in configs; extraction failures: 0

## Events this module publishes

- none found by extraction

## Vanilla methods this module modifies

One row per (injection, selector). `resolution` says how the selector matched the processed jar; `points` are the @At targets with their own resolution.

| vanilla method | descriptor | resolution | injector | points | env | priority | handler |
|---|---|---|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.renderer.block.FluidRenderer|FluidRenderer]].`tesselate` | `(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/client/renderer/block/FluidRenderer$Output;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/material/FluidState;)V` | name_only | @Inject | HEAD | client | 1000 (default) | `FluidRendererMixin.onHeadRender` |
| [[40-Interfaces/net.minecraft.client.renderer.block.FluidRenderer|FluidRenderer]].`tesselate` | `(Lnet/minecraft/client/renderer/block/BlockAndTintGetter;Lnet/minecraft/core/BlockPos;Lnet/minecraft/client/renderer/block/FluidRenderer$Output;Lnet/minecraft/world/level/block/state/BlockState;Lnet/minecraft/world/level/material/FluidState;)V` | name_only | @ModifyExpressionValue | MIXINEXTRAS:EXPRESSION (selector_unsupported) | client | 1000 (default) | `FluidRendererMixin.modifyNonOverlayCheck` |
| [[40-Interfaces/net.minecraft.client.renderer.block.FluidStateModelSet|FluidStateModelSet]].`bake` | `(Lnet/minecraft/client/resources/model/sprite/MaterialBaker;)Ljava/util/Map;` | name_only | @WrapMethod | - | client | 1000 (default) | `FluidStateModelSetMixin.bake` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.client.render.fluid.v1.FluidRenderHandler|FluidRenderHandler]] (interface, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.render.fluid.v1.FluidRendering|FluidRendering]] (class, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.render.fluid.v1.FluidRenderingRegistry|FluidRenderingRegistry]] (class, 8 members)

## What this establishes, and does not

- Injection targets, points and API signatures are `direct_reference`: read from the class files.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- How two injections compose is `executed_transformation` evidence in [[30-Mechanisms/Transformation_Tests]], not established per module.
- Nothing here is `observed`. No game ran.
