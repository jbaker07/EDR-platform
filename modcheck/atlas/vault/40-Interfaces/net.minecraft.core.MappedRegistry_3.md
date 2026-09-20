---
type: "interface"
fqcn: "net.minecraft.core.MappedRegistry$3"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.MappedRegistry$3

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `apply` | `@Inject at INVOKE Lnet/minecraft/core/MappedRegistry;refreshTagsInHolders()V` | both | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |

## Declared members (10, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
class net.minecraft.core.MappedRegistry$3 implements net.minecraft.core.Registry$PendingTags<T> {
    final java.util.Map val$pendingContents;
    final net.minecraft.core.HolderLookup$RegistryLookup val$patchedHolder;
    final com.google.common.collect.ImmutableMap val$pendingTags;
    final net.minecraft.core.MappedRegistry this$0;
    net.minecraft.core.MappedRegistry$3(net.minecraft.core.MappedRegistry, java.util.Map, net.minecraft.core.HolderLookup$RegistryLookup, com.google.common.collect.ImmutableMap);
    public net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<? extends T>> key();
    public int size();
    public net.minecraft.core.HolderLookup$RegistryLookup<T> lookup();
    public void apply();
    private static void lambda$apply$0(java.util.Map, net.minecraft.tags.TagKey, net.minecraft.core.HolderSet$Named);
}
```
