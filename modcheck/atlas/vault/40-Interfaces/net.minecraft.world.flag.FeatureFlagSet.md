---
type: "interface"
fqcn: "net.minecraft.world.flag.FeatureFlagSet"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.flag.FeatureFlagSet

System: [[20-Systems/net.minecraft.world.flag|net.minecraft.world.flag]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `isSubsetOf(Lnet/minecraft/world/flag/FeatureFlagSet;)Z` | `` | unknown | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |

## Declared members (19, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.world.flag.FeatureFlagSet {
    private static final net.minecraft.world.flag.FeatureFlagSet EMPTY;
    public static final int MAX_CONTAINER_SIZE;
    private final net.minecraft.world.flag.FeatureFlagUniverse universe;
    private final long mask;
    private net.minecraft.world.flag.FeatureFlagSet(net.minecraft.world.flag.FeatureFlagUniverse, long);
    static net.minecraft.world.flag.FeatureFlagSet create(net.minecraft.world.flag.FeatureFlagUniverse, java.util.Collection<net.minecraft.world.flag.FeatureFlag>);
    public static net.minecraft.world.flag.FeatureFlagSet of();
    public static net.minecraft.world.flag.FeatureFlagSet of(net.minecraft.world.flag.FeatureFlag);
    public static net.minecraft.world.flag.FeatureFlagSet of(net.minecraft.world.flag.FeatureFlag, net.minecraft.world.flag.FeatureFlag...);
    private static long computeMask(net.minecraft.world.flag.FeatureFlagUniverse, long, java.lang.Iterable<net.minecraft.world.flag.FeatureFlag>);
    public boolean contains(net.minecraft.world.flag.FeatureFlag);
    public boolean isEmpty();
    public boolean isSubsetOf(net.minecraft.world.flag.FeatureFlagSet);
    public boolean intersects(net.minecraft.world.flag.FeatureFlagSet);
    public net.minecraft.world.flag.FeatureFlagSet join(net.minecraft.world.flag.FeatureFlagSet);
    public net.minecraft.world.flag.FeatureFlagSet subtract(net.minecraft.world.flag.FeatureFlagSet);
    public boolean equals(java.lang.Object);
    public int hashCode();
    static {};
}
```
