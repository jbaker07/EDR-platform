---
type: "interface"
fqcn: "net.minecraft.nbt.StringTag"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.nbt.StringTag

System: [[20-Systems/net.minecraft.nbt|net.minecraft.nbt]]

`record` public final; extends `java/lang/Record`; implements `net/minecraft/nbt/PrimitiveTag`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `value` | `()Ljava/lang/String;` | exact | invokevirtual@174 in `RegistryCustomContentState.fromNbt` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `valueOf` | `(Ljava/lang/String;)Lnet/minecraft/nbt/StringTag;` | exact | invokestatic@113 in `RegistryCustomContentState.toNbt` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (8 fields, 21 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final value : Ljava/lang/String;
private static final SELF_SIZE_IN_BYTES : I
public static final TYPE : Lnet/minecraft/nbt/TagType;
private static final EMPTY : Lnet/minecraft/nbt/StringTag;
private static final DOUBLE_QUOTE : C
private static final SINGLE_QUOTE : C
private static final ESCAPE : C
private static final NOT_SET : C
public <init>(Ljava/lang/String;)V
public static skipString(Ljava/io/DataInput;)V
public static valueOf(Ljava/lang/String;)Lnet/minecraft/nbt/StringTag;
public write(Ljava/io/DataOutput;)V
public sizeInBytes()I
public getId()B
public getType()Lnet/minecraft/nbt/TagType;
public toString()Ljava/lang/String;
public copy()Lnet/minecraft/nbt/StringTag;
public asString()Ljava/util/Optional;
public accept(Lnet/minecraft/nbt/TagVisitor;)V
public static quoteAndEscape(Ljava/lang/String;)Ljava/lang/String;
public static quoteAndEscape(Ljava/lang/String;Ljava/lang/StringBuilder;)V
public static escapeWithoutQuotes(Ljava/lang/String;)Ljava/lang/String;
public static escapeWithoutQuotes(Ljava/lang/String;Ljava/lang/StringBuilder;)V
public accept(Lnet/minecraft/nbt/StreamTagVisitor;)Lnet/minecraft/nbt/StreamTagVisitor$ValueResult;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public value()Ljava/lang/String;
public synthetic copy()Lnet/minecraft/nbt/Tag;
static <clinit>()V
```
