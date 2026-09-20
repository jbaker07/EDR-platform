---
type: "interface"
fqcn: "net.minecraft.world.level.chunk.LevelChunkSection$1BlockCounter"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.chunk.LevelChunkSection$1BlockCounter

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` ; extends `java/lang/Object`; implements `net/minecraft/world/level/chunk/PalettedContainer$CountConsumer`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| wraps | `accept` | `(Lnet/minecraft/world/level/block/state/BlockState;I)V` | exact | @Redirect at ['INVOKE'] | both | [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]] | direct_reference |

## Declared members (4 fields, 3 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public nonEmptyBlockCount : I
public fluidCount : I
public tickingBlockCount : I
public tickingFluidCount : I
 <init>(Lnet/minecraft/world/level/chunk/LevelChunkSection;)V
public accept(Lnet/minecraft/world/level/block/state/BlockState;I)V
public synthetic accept(Ljava/lang/Object;I)V
```
