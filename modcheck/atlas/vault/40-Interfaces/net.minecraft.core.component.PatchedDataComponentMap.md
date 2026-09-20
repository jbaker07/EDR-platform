---
type: "interface"
fqcn: "net.minecraft.core.component.PatchedDataComponentMap"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.component.PatchedDataComponentMap

System: [[20-Systems/net.minecraft.core.component|net.minecraft.core.component]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `fromPatch(Lnet/minecraft/core/component/DataComponentMap;Lnet/minecra` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| calls | `fromPatch(Lnet/minecraft/core/component/DataComponentMap;Lnet/minecra` | `` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (27, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.core.component.PatchedDataComponentMap implements net.minecraft.core.component.DataComponentMap {
    private final net.minecraft.core.component.DataComponentMap prototype;
    private it.unimi.dsi.fastutil.objects.Reference2ObjectMap<net.minecraft.core.component.DataComponentType<?>, java.lang.Object> patch;
    private boolean copyOnWrite;
    public net.minecraft.core.component.PatchedDataComponentMap(net.minecraft.core.component.DataComponentMap);
    private net.minecraft.core.component.PatchedDataComponentMap(net.minecraft.core.component.DataComponentMap, it.unimi.dsi.fastutil.objects.Reference2ObjectMap<net.minecraft.core.component.DataComponentType<?>, java.lang.Object>, boolean);
    public static net.minecraft.core.component.PatchedDataComponentMap fromPatch(net.minecraft.core.component.DataComponentMap, net.minecraft.core.component.DataComponentPatch);
    private static boolean isPatchSanitized(net.minecraft.core.component.DataComponentMap, it.unimi.dsi.fastutil.objects.Reference2ObjectMap<net.minecraft.core.component.DataComponentType<?>, java.lang.Object>);
    public <T> T get(net.minecraft.core.component.DataComponentType<? extends T>);
    public boolean hasNonDefault(net.minecraft.core.component.DataComponentType<?>);
    public <T> T set(net.minecraft.core.component.DataComponentType<T>, T);
    public <T> T set(net.minecraft.core.component.TypedDataComponent<T>);
    public <T> T remove(net.minecraft.core.component.DataComponentType<? extends T>);
    public void applyPatch(net.minecraft.core.component.DataComponentPatch);
    private void applyPatch(net.minecraft.core.component.DataComponentType<?>, java.lang.Object);
    public void restorePatch(net.minecraft.core.component.DataComponentPatch);
    public void clearPatch();
    public void setAll(net.minecraft.core.component.DataComponentMap);
    private void ensureMapOwnership();
    public java.util.Set<net.minecraft.core.component.DataComponentType<?>> keySet();
    public java.util.Iterator<net.minecraft.core.component.TypedDataComponent<?>> iterator();
    public int size();
    public net.minecraft.core.component.DataComponentPatch asPatch();
    public net.minecraft.core.component.PatchedDataComponentMap copy();
    public net.minecraft.core.component.DataComponentMap toImmutableMap();
    public boolean equals(java.lang.Object);
    public int hashCode();
    public java.lang.String toString();
}
```
