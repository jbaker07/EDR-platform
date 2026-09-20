---
type: "interface"
fqcn: "net.minecraft.network.protocol.login.custom.CustomQueryPayload"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.protocol.login.custom.CustomQueryPayload

System: [[20-Systems/net.minecraft.network.protocol|net.minecraft.network.protocol]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `id()Lnet/minecraft/resources/Identifier;` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `id()Lnet/minecraft/resources/Identifier;` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (2, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.network.protocol.login.custom.CustomQueryPayload {
    public abstract net.minecraft.resources.Identifier id();
    public abstract void write(net.minecraft.network.FriendlyByteBuf);
}
```
