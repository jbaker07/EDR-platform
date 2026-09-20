---
type: "interface"
fqcn: "net.minecraft.core.component.DataComponentInitializers$1"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.component.DataComponentInitializers$1

System: [[20-Systems/net.minecraft.core.component|net.minecraft.core.component]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `<init>` | `@Inject at TAIL` | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| injects_into | `apply` | `@Inject at RETURN` | both | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (7, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
class net.minecraft.core.component.DataComponentInitializers$1 implements net.minecraft.core.component.DataComponentInitializers$PendingComponents<T> {
    final net.minecraft.resources.ResourceKey val$registryKey;
    final java.util.List val$entries;
    net.minecraft.core.component.DataComponentInitializers$1(net.minecraft.resources.ResourceKey, java.util.List);
    public net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<? extends T>> key();
    public void forEach(java.util.function.BiConsumer<net.minecraft.core.Holder$Reference<T>, net.minecraft.core.component.DataComponentMap>);
    public void apply();
    private static void lambda$forEach$0(java.util.function.BiConsumer, net.minecraft.core.component.DataComponentInitializers$BakedEntry);
}
```
