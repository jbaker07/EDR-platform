---
type: "interface"
fqcn: "net.minecraft.nbt.StringTag"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.nbt.StringTag

System: [[20-Systems/net.minecraft.nbt|net.minecraft.nbt]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `value()Ljava/lang/String;` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `valueOf(Ljava/lang/String;)Lnet/minecraft/nbt/StringTag;` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (29, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.nbt.StringTag extends java.lang.Record implements net.minecraft.nbt.PrimitiveTag {
    private final java.lang.String value;
    private static final int SELF_SIZE_IN_BYTES;
    public static final net.minecraft.nbt.TagType<net.minecraft.nbt.StringTag> TYPE;
    private static final net.minecraft.nbt.StringTag EMPTY;
    private static final char DOUBLE_QUOTE;
    private static final char SINGLE_QUOTE;
    private static final char ESCAPE;
    private static final char NOT_SET;
    public net.minecraft.nbt.StringTag(java.lang.String);
    public static void skipString(java.io.DataInput) throws java.io.IOException;
    public static net.minecraft.nbt.StringTag valueOf(java.lang.String);
    public void write(java.io.DataOutput) throws java.io.IOException;
    public int sizeInBytes();
    public byte getId();
    public net.minecraft.nbt.TagType<net.minecraft.nbt.StringTag> getType();
    public java.lang.String toString();
    public net.minecraft.nbt.StringTag copy();
    public java.util.Optional<java.lang.String> asString();
    public void accept(net.minecraft.nbt.TagVisitor);
    public static java.lang.String quoteAndEscape(java.lang.String);
    public static void quoteAndEscape(java.lang.String, java.lang.StringBuilder);
    public static java.lang.String escapeWithoutQuotes(java.lang.String);
    public static void escapeWithoutQuotes(java.lang.String, java.lang.StringBuilder);
    public net.minecraft.nbt.StreamTagVisitor$ValueResult accept(net.minecraft.nbt.StreamTagVisitor);
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public java.lang.String value();
    public net.minecraft.nbt.Tag copy();
    static {};
}
```
