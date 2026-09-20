---
type: "interface"
fqcn: "net.minecraft.client.resources.model.BlockStateModelLoader$LoadedModels"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.resources.model.BlockStateModelLoader$LoadedModels

System: [[20-Systems/net.minecraft.client.resources|net.minecraft.client.resources]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Ljava/util/Map;)V` | `` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |
| calls | `models()Ljava/util/Map;` | `` | unknown | [[30-Mechanisms/fabric-model-loading-api-v1|fabric-model-loading-api-v1]] | direct_reference |

## Declared members (6, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.client.resources.model.BlockStateModelLoader$LoadedModels extends java.lang.Record {
    private final java.util.Map<net.minecraft.world.level.block.state.BlockState, net.minecraft.client.renderer.block.dispatch.BlockStateModel$UnbakedRoot> models;
    public net.minecraft.client.resources.model.BlockStateModelLoader$LoadedModels(java.util.Map<net.minecraft.world.level.block.state.BlockState, net.minecraft.client.renderer.block.dispatch.BlockStateModel$UnbakedRoot>);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public java.util.Map<net.minecraft.world.level.block.state.BlockState, net.minecraft.client.renderer.block.dispatch.BlockStateModel$UnbakedRoot> models();
}
```
