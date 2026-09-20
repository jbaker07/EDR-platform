---
type: "interface"
fqcn: "net.minecraft.core.component.DataComponentPatch$Builder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.component.DataComponentPatch$Builder

System: [[20-Systems/net.minecraft.core.component|net.minecraft.core.component]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `build()Lnet/minecraft/core/component/DataComponentPatch;` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `build()Lnet/minecraft/core/component/DataComponentPatch;` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `build()Lnet/minecraft/core/component/DataComponentPatch;` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `set(Lnet/minecraft/core/component/DataComponentType;Ljava/lang/` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `set(Ljava/lang/Iterable;)Lnet/minecraft/core/component/DataComp` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `set(Lnet/minecraft/core/component/DataComponentType;Ljava/lang/` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (7, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.core.component.DataComponentPatch$Builder {
    private final it.unimi.dsi.fastutil.objects.Reference2ObjectMap<net.minecraft.core.component.DataComponentType<?>, java.lang.Object> map;
    private net.minecraft.core.component.DataComponentPatch$Builder();
    public <T> net.minecraft.core.component.DataComponentPatch$Builder set(net.minecraft.core.component.DataComponentType<T>, T);
    public <T> net.minecraft.core.component.DataComponentPatch$Builder remove(net.minecraft.core.component.DataComponentType<T>);
    public <T> net.minecraft.core.component.DataComponentPatch$Builder set(net.minecraft.core.component.TypedDataComponent<T>);
    public <T> net.minecraft.core.component.DataComponentPatch$Builder set(java.lang.Iterable<net.minecraft.core.component.TypedDataComponent<?>>);
    public net.minecraft.core.component.DataComponentPatch build();
}
```
