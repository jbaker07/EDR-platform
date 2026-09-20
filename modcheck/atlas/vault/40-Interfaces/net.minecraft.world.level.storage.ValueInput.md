---
type: "interface"
fqcn: "net.minecraft.world.level.storage.ValueInput"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.storage.ValueInput

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `read(Ljava/lang/String;Lcom/mojang/serialization/Codec;)Ljava/ut` | `` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |

## Declared members (21, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.world.level.storage.ValueInput {
    public abstract <T> java.util.Optional<T> read(java.lang.String, com.mojang.serialization.Codec<T>);
    public abstract <T> java.util.Optional<T> read(com.mojang.serialization.MapCodec<T>);
    public abstract java.util.Optional<net.minecraft.world.level.storage.ValueInput> child(java.lang.String);
    public abstract net.minecraft.world.level.storage.ValueInput childOrEmpty(java.lang.String);
    public abstract java.util.Optional<net.minecraft.world.level.storage.ValueInput$ValueInputList> childrenList(java.lang.String);
    public abstract net.minecraft.world.level.storage.ValueInput$ValueInputList childrenListOrEmpty(java.lang.String);
    public abstract <T> java.util.Optional<net.minecraft.world.level.storage.ValueInput$TypedInputList<T>> list(java.lang.String, com.mojang.serialization.Codec<T>);
    public abstract <T> net.minecraft.world.level.storage.ValueInput$TypedInputList<T> listOrEmpty(java.lang.String, com.mojang.serialization.Codec<T>);
    public abstract boolean getBooleanOr(java.lang.String, boolean);
    public abstract byte getByteOr(java.lang.String, byte);
    public abstract int getShortOr(java.lang.String, short);
    public abstract java.util.Optional<java.lang.Integer> getInt(java.lang.String);
    public abstract int getIntOr(java.lang.String, int);
    public abstract long getLongOr(java.lang.String, long);
    public abstract java.util.Optional<java.lang.Long> getLong(java.lang.String);
    public abstract float getFloatOr(java.lang.String, float);
    public abstract double getDoubleOr(java.lang.String, double);
    public abstract java.util.Optional<java.lang.String> getString(java.lang.String);
    public abstract java.lang.String getStringOr(java.lang.String, java.lang.String);
    public abstract java.util.Optional<int[]> getIntArray(java.lang.String);
    public abstract net.minecraft.core.HolderLookup$Provider lookup();
}
```
