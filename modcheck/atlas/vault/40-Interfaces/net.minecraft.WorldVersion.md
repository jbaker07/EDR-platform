---
type: "interface"
fqcn: "net.minecraft.WorldVersion"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.WorldVersion

System: [[20-Systems/net.minecraft|net.minecraft]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `packVersion(Lnet/minecraft/server/packs/PackType;)Lnet/minecraft/server` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (7, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.WorldVersion {
    public abstract net.minecraft.world.level.storage.DataVersion dataVersion();
    public abstract java.lang.String id();
    public abstract java.lang.String name();
    public abstract int protocolVersion();
    public abstract net.minecraft.server.packs.metadata.pack.PackFormat packVersion(net.minecraft.server.packs.PackType);
    public abstract java.util.Date buildTime();
    public abstract boolean stable();
}
```
