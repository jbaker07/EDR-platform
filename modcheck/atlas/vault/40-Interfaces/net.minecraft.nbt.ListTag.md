---
type: "interface"
fqcn: "net.minecraft.nbt.ListTag"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.nbt.ListTag

System: [[20-Systems/net.minecraft.nbt|net.minecraft.nbt]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"()V` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `add(Ljava/lang/Object;)Z` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `iterator()Ljava/util/Iterator;` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (59, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.nbt.ListTag extends java.util.AbstractList<net.minecraft.nbt.Tag> implements net.minecraft.nbt.CollectionTag {
    private static final java.lang.String WRAPPER_MARKER;
    private static final int SELF_SIZE_IN_BYTES;
    public static final net.minecraft.nbt.TagType<net.minecraft.nbt.ListTag> TYPE;
    private final java.util.List<net.minecraft.nbt.Tag> list;
    public net.minecraft.nbt.ListTag();
    net.minecraft.nbt.ListTag(java.util.List<net.minecraft.nbt.Tag>);
    private static net.minecraft.nbt.Tag tryUnwrap(net.minecraft.nbt.CompoundTag);
    private static boolean isWrapper(net.minecraft.nbt.CompoundTag);
    private static net.minecraft.nbt.Tag wrapIfNeeded(byte, net.minecraft.nbt.Tag);
    private static net.minecraft.nbt.CompoundTag wrapElement(net.minecraft.nbt.Tag);
    public void write(java.io.DataOutput) throws java.io.IOException;
    byte identifyRawElementType();
    public void addAndUnwrap(net.minecraft.nbt.Tag);
    public int sizeInBytes();
    public byte getId();
    public net.minecraft.nbt.TagType<net.minecraft.nbt.ListTag> getType();
    public java.lang.String toString();
    public net.minecraft.nbt.Tag remove(int);
    public boolean isEmpty();
    public java.util.Optional<net.minecraft.nbt.CompoundTag> getCompound(int);
    public net.minecraft.nbt.CompoundTag getCompoundOrEmpty(int);
    public java.util.Optional<net.minecraft.nbt.ListTag> getList(int);
    public net.minecraft.nbt.ListTag getListOrEmpty(int);
    public java.util.Optional<java.lang.Short> getShort(int);
    public short getShortOr(int, short);
    public java.util.Optional<java.lang.Integer> getInt(int);
    public int getIntOr(int, int);
    public java.util.Optional<int[]> getIntArray(int);
    public java.util.Optional<long[]> getLongArray(int);
    public java.util.Optional<java.lang.Double> getDouble(int);
    public double getDoubleOr(int, double);
    public java.util.Optional<java.lang.Float> getFloat(int);
    public float getFloatOr(int, float);
    public java.util.Optional<java.lang.String> getString(int);
    public java.lang.String getStringOr(int, java.lang.String);
    private net.minecraft.nbt.Tag getNullable(int);
    private java.util.Optional<net.minecraft.nbt.Tag> getOptional(int);
    public int size();
    public net.minecraft.nbt.Tag get(int);
    public net.minecraft.nbt.Tag set(int, net.minecraft.nbt.Tag);
    public void add(int, net.minecraft.nbt.Tag);
    public boolean setTag(int, net.minecraft.nbt.Tag);
    public boolean addTag(int, net.minecraft.nbt.Tag);
    public net.minecraft.nbt.ListTag copy();
    public java.util.Optional<net.minecraft.nbt.ListTag> asList();
    public boolean equals(java.lang.Object);
    public int hashCode();
    public java.util.stream.Stream<net.minecraft.nbt.Tag> stream();
    public java.util.stream.Stream<net.minecraft.nbt.CompoundTag> compoundStream();
    public void accept(net.minecraft.nbt.TagVisitor);
    public void clear();
    public net.minecraft.nbt.StreamTagVisitor$ValueResult accept(net.minecraft.nbt.StreamTagVisitor);
    public java.lang.Object remove(int);
    public void add(int, java.lang.Object);
    public java.lang.Object set(int, java.lang.Object);
    public java.lang.Object get(int);
    public net.minecraft.nbt.Tag copy();
    private static void lambda$compoundStream$0(net.minecraft.nbt.Tag, java.util.function.Consumer);
    static {};
}
```
