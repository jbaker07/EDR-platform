---
type: "interface"
fqcn: "net.minecraft.client.renderer.entity.EntityRendererProvider"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.entity.EntityRendererProvider

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `create(Lnet/minecraft/client/renderer/entity/EntityRendererProvide` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (1, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.client.renderer.entity.EntityRendererProvider<T extends net.minecraft.world.entity.Entity> {
    public abstract net.minecraft.client.renderer.entity.EntityRenderer<T, ?> create(net.minecraft.client.renderer.entity.EntityRendererProvider$Context);
}
```
