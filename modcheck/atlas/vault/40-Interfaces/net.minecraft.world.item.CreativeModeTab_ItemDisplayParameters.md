---
type: "interface"
fqcn: "net.minecraft.world.item.CreativeModeTab$ItemDisplayParameters"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.CreativeModeTab$ItemDisplayParameters

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `hasPermissions()Z` | `` | both | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `hasPermissions()Z` | `` | client | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |

## Declared members (11, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.world.item.CreativeModeTab$ItemDisplayParameters extends java.lang.Record {
    private final net.minecraft.world.flag.FeatureFlagSet enabledFeatures;
    private final boolean hasPermissions;
    private final net.minecraft.core.HolderLookup$Provider holders;
    public net.minecraft.world.item.CreativeModeTab$ItemDisplayParameters(net.minecraft.world.flag.FeatureFlagSet, boolean, net.minecraft.core.HolderLookup$Provider);
    public boolean needsUpdate(net.minecraft.world.flag.FeatureFlagSet, boolean, net.minecraft.core.HolderLookup$Provider);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.world.flag.FeatureFlagSet enabledFeatures();
    public boolean hasPermissions();
    public net.minecraft.core.HolderLookup$Provider holders();
}
```
