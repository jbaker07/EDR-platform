---
type: "interface"
fqcn: "net.minecraft.core.LayeredRegistryAccess"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.LayeredRegistryAccess

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `compositeAccess()Lnet/minecraft/core/RegistryAccess$Frozen;` | `` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `compositeAccess()Lnet/minecraft/core/RegistryAccess$Frozen;` | `` | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `compositeAccess()Lnet/minecraft/core/RegistryAccess$Frozen;` | `` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `compositeAccess()Lnet/minecraft/core/RegistryAccess$Frozen;` | `` | client | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getLayer(Ljava/lang/Object;)Lnet/minecraft/core/RegistryAccess$Froze` | `` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |

## Declared members (17, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.core.LayeredRegistryAccess<T> {
    private final java.util.List<T> keys;
    private final java.util.List<net.minecraft.core.RegistryAccess$Frozen> values;
    private final net.minecraft.core.RegistryAccess$Frozen composite;
    public net.minecraft.core.LayeredRegistryAccess(java.util.List<T>);
    private net.minecraft.core.LayeredRegistryAccess(java.util.List<T>, java.util.List<net.minecraft.core.RegistryAccess$Frozen>);
    private int getLayerIndexOrThrow(T);
    public net.minecraft.core.RegistryAccess$Frozen getLayer(T);
    public net.minecraft.core.RegistryAccess$Frozen getAccessForLoading(T);
    public net.minecraft.core.RegistryAccess$Frozen getAccessFrom(T);
    private net.minecraft.core.RegistryAccess$Frozen getCompositeAccessForLayers(int, int);
    public net.minecraft.core.LayeredRegistryAccess<T> replaceFrom(T, net.minecraft.core.RegistryAccess$Frozen...);
    public net.minecraft.core.LayeredRegistryAccess<T> replaceFrom(T, java.util.List<net.minecraft.core.RegistryAccess$Frozen>);
    public net.minecraft.core.RegistryAccess$Frozen compositeAccess();
    private static java.util.Map<net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<?>>, net.minecraft.core.Registry<?>> collectRegistries(java.util.stream.Stream<? extends net.minecraft.core.RegistryAccess>);
    private static void lambda$collectRegistries$0(java.util.Map, net.minecraft.core.RegistryAccess);
    private static void lambda$collectRegistries$1(java.util.Map, net.minecraft.core.RegistryAccess$RegistryEntry);
    private static java.util.List lambda$new$0(java.util.List);
}
```
