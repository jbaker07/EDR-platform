---
type: "interface"
fqcn: "net.minecraft.core.component.DataComponentMap$Builder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.component.DataComponentMap$Builder

System: [[20-Systems/net.minecraft.core.component|net.minecraft.core.component]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `addAll(Lnet/minecraft/core/component/DataComponentMap;)Lnet/minecr` | `` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |
| calls | `build()Lnet/minecraft/core/component/DataComponentMap;` | `` | unknown | [[30-Mechanisms/fabric-item-api-v1|fabric-item-api-v1]] | direct_reference |

## Declared members (10, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.core.component.DataComponentMap$Builder {
    private final it.unimi.dsi.fastutil.objects.Reference2ObjectMap<net.minecraft.core.component.DataComponentType<?>, java.lang.Object> map;
    private java.util.function.Consumer<net.minecraft.core.component.DataComponentMap> validator;
    private net.minecraft.core.component.DataComponentMap$Builder();
    public <T> net.minecraft.core.component.DataComponentMap$Builder set(net.minecraft.core.component.DataComponentType<T>, T);
    <T> void setUnchecked(net.minecraft.core.component.DataComponentType<T>, java.lang.Object);
    public net.minecraft.core.component.DataComponentMap$Builder addAll(net.minecraft.core.component.DataComponentMap);
    public net.minecraft.core.component.DataComponentMap$Builder addValidator(java.util.function.Consumer<net.minecraft.core.component.DataComponentMap>);
    public net.minecraft.core.component.DataComponentMap build();
    private static net.minecraft.core.component.DataComponentMap buildFromMapTrusted(java.util.Map<net.minecraft.core.component.DataComponentType<?>, java.lang.Object>);
    private static void lambda$new$0(net.minecraft.core.component.DataComponentMap);
}
```
