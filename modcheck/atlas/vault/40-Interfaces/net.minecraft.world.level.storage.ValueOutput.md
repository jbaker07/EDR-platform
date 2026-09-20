---
type: "interface"
fqcn: "net.minecraft.world.level.storage.ValueOutput"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.storage.ValueOutput

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `store(Ljava/lang/String;Lcom/mojang/serialization/Codec;Ljava/lan` | `` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |

## Declared members (17, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.world.level.storage.ValueOutput {
    public abstract <T> void store(java.lang.String, com.mojang.serialization.Codec<T>, T);
    public abstract <T> void storeNullable(java.lang.String, com.mojang.serialization.Codec<T>, T);
    public abstract <T> void store(com.mojang.serialization.MapCodec<T>, T);
    public abstract void putBoolean(java.lang.String, boolean);
    public abstract void putByte(java.lang.String, byte);
    public abstract void putShort(java.lang.String, short);
    public abstract void putInt(java.lang.String, int);
    public abstract void putLong(java.lang.String, long);
    public abstract void putFloat(java.lang.String, float);
    public abstract void putDouble(java.lang.String, double);
    public abstract void putString(java.lang.String, java.lang.String);
    public abstract void putIntArray(java.lang.String, int[]);
    public abstract net.minecraft.world.level.storage.ValueOutput child(java.lang.String);
    public abstract net.minecraft.world.level.storage.ValueOutput$ValueOutputList childrenList(java.lang.String);
    public abstract <T> net.minecraft.world.level.storage.ValueOutput$TypedOutputList<T> list(java.lang.String, com.mojang.serialization.Codec<T>);
    public abstract void discard(java.lang.String);
    public abstract boolean isEmpty();
}
```
