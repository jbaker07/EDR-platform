---
type: "interface"
fqcn: "net.minecraft.world.level.storage.TagValueInput"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.storage.TagValueInput

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `create(Lnet/minecraft/util/ProblemReporter;Lnet/minecraft/core/Hol` | `` | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |

## Declared members (34, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.storage.TagValueInput implements net.minecraft.world.level.storage.ValueInput {
    private final net.minecraft.util.ProblemReporter problemReporter;
    private final net.minecraft.world.level.storage.ValueInputContextHelper context;
    private final net.minecraft.nbt.CompoundTag input;
    private net.minecraft.world.level.storage.TagValueInput(net.minecraft.util.ProblemReporter, net.minecraft.world.level.storage.ValueInputContextHelper, net.minecraft.nbt.CompoundTag);
    public static net.minecraft.world.level.storage.ValueInput create(net.minecraft.util.ProblemReporter, net.minecraft.core.HolderLookup$Provider, net.minecraft.nbt.CompoundTag);
    public static net.minecraft.world.level.storage.ValueInput$ValueInputList create(net.minecraft.util.ProblemReporter, net.minecraft.core.HolderLookup$Provider, java.util.List<net.minecraft.nbt.CompoundTag>);
    public <T> java.util.Optional<T> read(java.lang.String, com.mojang.serialization.Codec<T>);
    public <T> java.util.Optional<T> read(com.mojang.serialization.MapCodec<T>);
    private <T extends net.minecraft.nbt.Tag> T getOptionalTypedTag(java.lang.String, net.minecraft.nbt.TagType<T>);
    private net.minecraft.nbt.NumericTag getNumericTag(java.lang.String);
    public java.util.Optional<net.minecraft.world.level.storage.ValueInput> child(java.lang.String);
    public net.minecraft.world.level.storage.ValueInput childOrEmpty(java.lang.String);
    public java.util.Optional<net.minecraft.world.level.storage.ValueInput$ValueInputList> childrenList(java.lang.String);
    public net.minecraft.world.level.storage.ValueInput$ValueInputList childrenListOrEmpty(java.lang.String);
    public <T> java.util.Optional<net.minecraft.world.level.storage.ValueInput$TypedInputList<T>> list(java.lang.String, com.mojang.serialization.Codec<T>);
    public <T> net.minecraft.world.level.storage.ValueInput$TypedInputList<T> listOrEmpty(java.lang.String, com.mojang.serialization.Codec<T>);
    public boolean getBooleanOr(java.lang.String, boolean);
    public byte getByteOr(java.lang.String, byte);
    public int getShortOr(java.lang.String, short);
    public java.util.Optional<java.lang.Integer> getInt(java.lang.String);
    public int getIntOr(java.lang.String, int);
    public long getLongOr(java.lang.String, long);
    public java.util.Optional<java.lang.Long> getLong(java.lang.String);
    public float getFloatOr(java.lang.String, float);
    public double getDoubleOr(java.lang.String, double);
    public java.util.Optional<java.lang.String> getString(java.lang.String);
    public java.lang.String getStringOr(java.lang.String, java.lang.String);
    public java.util.Optional<int[]> getIntArray(java.lang.String);
    public net.minecraft.core.HolderLookup$Provider lookup();
    private net.minecraft.world.level.storage.ValueInput wrapChild(java.lang.String, net.minecraft.nbt.CompoundTag);
    private static net.minecraft.world.level.storage.ValueInput wrapChild(net.minecraft.util.ProblemReporter, net.minecraft.world.level.storage.ValueInputContextHelper, net.minecraft.nbt.CompoundTag);
    private net.minecraft.world.level.storage.ValueInput$ValueInputList wrapList(java.lang.String, net.minecraft.world.level.storage.ValueInputContextHelper, net.minecraft.nbt.ListTag);
    private <T> net.minecraft.world.level.storage.ValueInput$TypedInputList<T> wrapTypedList(java.lang.String, net.minecraft.nbt.ListTag, com.mojang.serialization.Codec<T>);
    private static com.mojang.serialization.DataResult lambda$read$0(com.mojang.serialization.MapCodec, com.mojang.serialization.DynamicOps, com.mojang.serialization.MapLike);
}
```
