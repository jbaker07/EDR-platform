---
type: "interface"
fqcn: "net.minecraft.world.level.saveddata.SavedDataType"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.saveddata.SavedDataType

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/resources/Identifier;Ljava/util/function/Sup` | `` | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `"<init>"(Lnet/minecraft/resources/Identifier;Ljava/util/function/Sup` | `` | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |

## Declared members (12, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.world.level.saveddata.SavedDataType<T extends net.minecraft.world.level.saveddata.SavedData> extends java.lang.Record {
    private final net.minecraft.resources.Identifier id;
    private final java.util.function.Supplier<T> constructor;
    private final com.mojang.serialization.Codec<T> codec;
    private final net.minecraft.util.datafix.DataFixTypes dataFixType;
    public net.minecraft.world.level.saveddata.SavedDataType(net.minecraft.resources.Identifier, java.util.function.Supplier<T>, com.mojang.serialization.Codec<T>, net.minecraft.util.datafix.DataFixTypes);
    public boolean equals(java.lang.Object);
    public int hashCode();
    public java.lang.String toString();
    public net.minecraft.resources.Identifier id();
    public java.util.function.Supplier<T> constructor();
    public com.mojang.serialization.Codec<T> codec();
    public net.minecraft.util.datafix.DataFixTypes dataFixType();
}
```
