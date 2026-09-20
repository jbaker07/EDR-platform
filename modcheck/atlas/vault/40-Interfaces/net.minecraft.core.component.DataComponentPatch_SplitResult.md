---
type: "interface"
fqcn: "net.minecraft.core.component.DataComponentPatch$SplitResult"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.component.DataComponentPatch$SplitResult

System: [[20-Systems/net.minecraft.core.component|net.minecraft.core.component]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `added()Lnet/minecraft/core/component/DataComponentMap;` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `added()Lnet/minecraft/core/component/DataComponentMap;` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `removed()Ljava/util/Set;` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `removed()Ljava/util/Set;` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (10, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.core.component.DataComponentPatch$SplitResult extends java.lang.Record {
    private final net.minecraft.core.component.DataComponentMap added;
    private final java.util.Set<net.minecraft.core.component.DataComponentType<?>> removed;
    public static final net.minecraft.core.component.DataComponentPatch$SplitResult EMPTY;
    public net.minecraft.core.component.DataComponentPatch$SplitResult(net.minecraft.core.component.DataComponentMap, java.util.Set<net.minecraft.core.component.DataComponentType<?>>);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public net.minecraft.core.component.DataComponentMap added();
    public java.util.Set<net.minecraft.core.component.DataComponentType<?>> removed();
    static {};
}
```
