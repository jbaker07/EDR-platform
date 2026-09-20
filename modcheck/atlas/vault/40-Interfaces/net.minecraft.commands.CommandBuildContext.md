---
type: "interface"
fqcn: "net.minecraft.commands.CommandBuildContext"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.commands.CommandBuildContext

System: [[20-Systems/net.minecraft.commands|net.minecraft.commands]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `simple(Lnet/minecraft/core/HolderLookup$Provider;Lnet/minecraft/wo` | `` | client | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |

## Declared members (2, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.commands.CommandBuildContext extends net.minecraft.core.HolderLookup$Provider {
    public static net.minecraft.commands.CommandBuildContext simple(net.minecraft.core.HolderLookup$Provider, net.minecraft.world.flag.FeatureFlagSet);
    public abstract net.minecraft.world.flag.FeatureFlagSet enabledFeatures();
}
```
