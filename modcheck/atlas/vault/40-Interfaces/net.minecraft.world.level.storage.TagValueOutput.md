---
type: "interface"
fqcn: "net.minecraft.world.level.storage.TagValueOutput"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.storage.TagValueOutput

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `buildResult()Lnet/minecraft/nbt/CompoundTag;` | `` | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `createWithContext(Lnet/minecraft/util/ProblemReporter;Lnet/minecraft/core/Hol` | `` | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |

## Declared members (27, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.storage.TagValueOutput implements net.minecraft.world.level.storage.ValueOutput {
    private final net.minecraft.util.ProblemReporter problemReporter;
    private final com.mojang.serialization.DynamicOps<net.minecraft.nbt.Tag> ops;
    private final net.minecraft.nbt.CompoundTag output;
    private net.minecraft.world.level.storage.TagValueOutput(net.minecraft.util.ProblemReporter, com.mojang.serialization.DynamicOps<net.minecraft.nbt.Tag>, net.minecraft.nbt.CompoundTag);
    public static net.minecraft.world.level.storage.TagValueOutput createWithContext(net.minecraft.util.ProblemReporter, net.minecraft.core.HolderLookup$Provider);
    public static net.minecraft.world.level.storage.TagValueOutput createWithoutContext(net.minecraft.util.ProblemReporter);
    public <T> void store(java.lang.String, com.mojang.serialization.Codec<T>, T);
    public <T> void storeNullable(java.lang.String, com.mojang.serialization.Codec<T>, T);
    public <T> void store(com.mojang.serialization.MapCodec<T>, T);
    public void putBoolean(java.lang.String, boolean);
    public void putByte(java.lang.String, byte);
    public void putShort(java.lang.String, short);
    public void putInt(java.lang.String, int);
    public void putLong(java.lang.String, long);
    public void putFloat(java.lang.String, float);
    public void putDouble(java.lang.String, double);
    public void putString(java.lang.String, java.lang.String);
    public void putIntArray(java.lang.String, int[]);
    private net.minecraft.util.ProblemReporter reporterForChild(java.lang.String);
    public net.minecraft.world.level.storage.ValueOutput child(java.lang.String);
    public net.minecraft.world.level.storage.ValueOutput$ValueOutputList childrenList(java.lang.String);
    public <T> net.minecraft.world.level.storage.ValueOutput$TypedOutputList<T> list(java.lang.String, com.mojang.serialization.Codec<T>);
    public void discard(java.lang.String);
    public boolean isEmpty();
    public net.minecraft.nbt.CompoundTag buildResult();
    private void lambda$store$1(net.minecraft.nbt.Tag);
    private void lambda$store$0(java.lang.String, net.minecraft.nbt.Tag);
}
```
