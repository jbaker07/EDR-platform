---
type: "interface"
fqcn: "net.minecraft.client.resources.model.ModelBaker"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.resources.model.ModelBaker

System: [[20-Systems/net.minecraft.client.resources|net.minecraft.client.resources]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `materials()Lnet/minecraft/client/resources/model/sprite/MaterialBaker` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (5, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.client.resources.model.ModelBaker {
    public abstract net.minecraft.client.resources.model.ResolvedModel getModel(net.minecraft.resources.Identifier);
    public abstract net.minecraft.client.renderer.block.dispatch.BlockStateModelPart missingBlockModelPart();
    public abstract net.minecraft.client.resources.model.sprite.MaterialBaker materials();
    public abstract net.minecraft.client.resources.model.ModelBaker$Interner interner();
    public abstract <T> T compute(net.minecraft.client.resources.model.ModelBaker$SharedOperationKey<T>);
}
```
