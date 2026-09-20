---
type: "interface"
fqcn: "net.minecraft.nbt.CompoundTag"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.nbt.CompoundTag

System: [[20-Systems/net.minecraft.nbt|net.minecraft.nbt]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"()V` | `` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `"<init>"()V` | `` | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `"<init>"()V` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `"<init>"()V` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `contains(Ljava/lang/String;)Z` | `` | both | [[30-Mechanisms/fabric-serialization-api-v1|fabric-serialization-api-v1]] | direct_reference |
| calls | `equals(Ljava/lang/Object;)Z` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `getByteArray(Ljava/lang/String;)Ljava/util/Optional;` | `` | both | [[30-Mechanisms/fabric-serialization-api-v1|fabric-serialization-api-v1]] | direct_reference |
| calls | `getCompound(Ljava/lang/String;)Ljava/util/Optional;` | `` | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `getCompound(Ljava/lang/String;)Ljava/util/Optional;` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getCompound(Ljava/lang/String;)Ljava/util/Optional;` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getIntOr(Ljava/lang/String;I)I` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getIntOr(Ljava/lang/String;I)I` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getList(Ljava/lang/String;)Ljava/util/Optional;` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getLongArray(Ljava/lang/String;)Ljava/util/Optional;` | `` | both | [[30-Mechanisms/fabric-serialization-api-v1|fabric-serialization-api-v1]] | direct_reference |
| calls | `isEmpty()Z` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `keySet()Ljava/util/Set;` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `keySet()Ljava/util/Set;` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `keySet()Ljava/util/Set;` | `` | both | [[30-Mechanisms/fabric-serialization-api-v1|fabric-serialization-api-v1]] | direct_reference |
| calls | `put(Ljava/lang/String;Lnet/minecraft/nbt/Tag;)Lnet/minecraft/nb` | `` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `put(Ljava/lang/String;Lnet/minecraft/nbt/Tag;)Lnet/minecraft/nb` | `` | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `put(Ljava/lang/String;Lnet/minecraft/nbt/Tag;)Lnet/minecraft/nb` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `put(Ljava/lang/String;Lnet/minecraft/nbt/Tag;)Lnet/minecraft/nb` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `putByteArray(Ljava/lang/String;[B)V` | `` | both | [[30-Mechanisms/fabric-serialization-api-v1|fabric-serialization-api-v1]] | direct_reference |
| calls | `putInt(Ljava/lang/String;I)V` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `putInt(Ljava/lang/String;I)V` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `putLongArray(Ljava/lang/String;[J)V` | `` | both | [[30-Mechanisms/fabric-serialization-api-v1|fabric-serialization-api-v1]] | direct_reference |
| calls | `remove(Ljava/lang/String;)Lnet/minecraft/nbt/Tag;` | `` | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |

## Declared members (86, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.nbt.CompoundTag implements net.minecraft.nbt.Tag {
    private static final org.slf4j.Logger LOGGER;
    public static final com.mojang.serialization.Codec<net.minecraft.nbt.CompoundTag> CODEC;
    private static final int SELF_SIZE_IN_BYTES;
    private static final int MAP_ENTRY_SIZE_IN_BYTES;
    public static final net.minecraft.nbt.TagType<net.minecraft.nbt.CompoundTag> TYPE;
    private final java.util.Map<java.lang.String, net.minecraft.nbt.Tag> tags;
    net.minecraft.nbt.CompoundTag(java.util.Map<java.lang.String, net.minecraft.nbt.Tag>);
    public net.minecraft.nbt.CompoundTag();
    public void write(java.io.DataOutput) throws java.io.IOException;
    public int sizeInBytes();
    public java.util.Set<java.lang.String> keySet();
    public java.util.Set<java.util.Map$Entry<java.lang.String, net.minecraft.nbt.Tag>> entrySet();
    public java.util.Collection<net.minecraft.nbt.Tag> values();
    public void forEach(java.util.function.BiConsumer<java.lang.String, net.minecraft.nbt.Tag>);
    public byte getId();
    public net.minecraft.nbt.TagType<net.minecraft.nbt.CompoundTag> getType();
    public int size();
    public net.minecraft.nbt.Tag put(java.lang.String, net.minecraft.nbt.Tag);
    public void putByte(java.lang.String, byte);
    public void putShort(java.lang.String, short);
    public void putInt(java.lang.String, int);
    public void putLong(java.lang.String, long);
    public void putFloat(java.lang.String, float);
    public void putDouble(java.lang.String, double);
    public void putString(java.lang.String, java.lang.String);
    public void putByteArray(java.lang.String, byte[]);
    public void putIntArray(java.lang.String, int[]);
    public void putLongArray(java.lang.String, long[]);
    public void putBoolean(java.lang.String, boolean);
    public net.minecraft.nbt.Tag get(java.lang.String);
    public boolean contains(java.lang.String);
    private java.util.Optional<net.minecraft.nbt.Tag> getOptional(java.lang.String);
    public java.util.Optional<java.lang.Byte> getByte(java.lang.String);
    public byte getByteOr(java.lang.String, byte);
    public java.util.Optional<java.lang.Short> getShort(java.lang.String);
    public short getShortOr(java.lang.String, short);
    public java.util.Optional<java.lang.Integer> getInt(java.lang.String);
    public int getIntOr(java.lang.String, int);
    public java.util.Optional<java.lang.Long> getLong(java.lang.String);
    public long getLongOr(java.lang.String, long);
    public java.util.Optional<java.lang.Float> getFloat(java.lang.String);
    public float getFloatOr(java.lang.String, float);
    public java.util.Optional<java.lang.Double> getDouble(java.lang.String);
    public double getDoubleOr(java.lang.String, double);
    public java.util.Optional<java.lang.String> getString(java.lang.String);
    public java.lang.String getStringOr(java.lang.String, java.lang.String);
    public java.util.Optional<byte[]> getByteArray(java.lang.String);
    public java.util.Optional<int[]> getIntArray(java.lang.String);
    public java.util.Optional<long[]> getLongArray(java.lang.String);
    public java.util.Optional<net.minecraft.nbt.CompoundTag> getCompound(java.lang.String);
    public net.minecraft.nbt.CompoundTag getCompoundOrEmpty(java.lang.String);
    public java.util.Optional<net.minecraft.nbt.ListTag> getList(java.lang.String);
    public net.minecraft.nbt.ListTag getListOrEmpty(java.lang.String);
    public java.util.Optional<java.lang.Boolean> getBoolean(java.lang.String);
    public boolean getBooleanOr(java.lang.String, boolean);
    public net.minecraft.nbt.Tag remove(java.lang.String);
    public java.lang.String toString();
    public boolean isEmpty();
    net.minecraft.nbt.CompoundTag shallowCopy();
    public net.minecraft.nbt.CompoundTag copy();
    public java.util.Optional<net.minecraft.nbt.CompoundTag> asCompound();
    public boolean equals(java.lang.Object);
    public int hashCode();
    private static void writeNamedTag(java.lang.String, net.minecraft.nbt.Tag, java.io.DataOutput) throws java.io.IOException;
    private static net.minecraft.nbt.Tag readNamedTagData(net.minecraft.nbt.TagType<?>, java.lang.String, java.io.DataInput, net.minecraft.nbt.NbtAccounter);
    public net.minecraft.nbt.CompoundTag merge(net.minecraft.nbt.CompoundTag);
    public void accept(net.minecraft.nbt.TagVisitor);
    public net.minecraft.nbt.StreamTagVisitor$ValueResult accept(net.minecraft.nbt.StreamTagVisitor);
    public <T> void store(java.lang.String, com.mojang.serialization.Codec<T>, T);
    public <T> void storeNullable(java.lang.String, com.mojang.serialization.Codec<T>, T);
    public <T> void store(java.lang.String, com.mojang.serialization.Codec<T>, com.mojang.serialization.DynamicOps<net.minecraft.nbt.Tag>, T);
    public <T> void storeNullable(java.lang.String, com.mojang.serialization.Codec<T>, com.mojang.serialization.DynamicOps<net.minecraft.nbt.Tag>, T);
    public <T> void store(com.mojang.serialization.MapCodec<T>, T);
    public <T> void store(com.mojang.serialization.MapCodec<T>, com.mojang.serialization.DynamicOps<net.minecraft.nbt.Tag>, T);
    public <T> java.util.Optional<T> read(java.lang.String, com.mojang.serialization.Codec<T>);
    public <T> java.util.Optional<T> read(java.lang.String, com.mojang.serialization.Codec<T>, com.mojang.serialization.DynamicOps<net.minecraft.nbt.Tag>);
    public <T> java.util.Optional<T> read(com.mojang.serialization.MapCodec<T>);
    public <T> java.util.Optional<T> read(com.mojang.serialization.MapCodec<T>, com.mojang.serialization.DynamicOps<net.minecraft.nbt.Tag>);
    public net.minecraft.nbt.Tag copy();
    private void lambda$read$1(java.lang.String);
    private static void lambda$read$0(java.lang.String, net.minecraft.nbt.Tag, java.lang.String);
    private static void lambda$copy$0(java.util.HashMap, java.lang.String, net.minecraft.nbt.Tag);
    private static com.mojang.serialization.Dynamic lambda$static$2(net.minecraft.nbt.CompoundTag);
    private static com.mojang.serialization.DataResult lambda$static$0(com.mojang.serialization.Dynamic);
    private static java.lang.String lambda$static$1(net.minecraft.nbt.Tag);
    static {};
}
```
