---
type: "interface"
fqcn: "net.minecraft.client.renderer.SubmitNodeCollector"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.SubmitNodeCollector

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `submitBlockModel(Lcom/mojang/blaze3d/vertex/PoseStack;Ljava/util/function/Fu` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `submitBreakingBlockModel(Lcom/mojang/blaze3d/vertex/PoseStack;Ljava/util/List;Lnet/f` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `submitItem(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/world/i` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `submitItem(Lcom/mojang/blaze3d/vertex/PoseStack;Lnet/minecraft/world/i` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (1, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.client.renderer.SubmitNodeCollector extends net.minecraft.client.renderer.OrderedSubmitNodeCollector {
    public abstract net.minecraft.client.renderer.OrderedSubmitNodeCollector order(int);
}
```
