---
type: "interface"
fqcn: "net.minecraft.nbt.ListTag"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.nbt.ListTag

System: [[20-Systems/net.minecraft.nbt|net.minecraft.nbt]]

`class` public final; extends `java/util/AbstractList`; implements `net/minecraft/nbt/CollectionTag`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `()V` | exact | invokespecial@62 in `RegistryCustomContentState.toNbt` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `add` | `(Ljava/lang/Object;)Z` | inherited_exact | invokevirtual@116 in `RegistryCustomContentState.toNbt` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `iterator` | `()Ljava/util/Iterator;` | inherited_exact | invokevirtual@130 in `RegistryCustomContentState.fromNbt` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (4 fields, 55 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final WRAPPER_MARKER : Ljava/lang/String;
private static final SELF_SIZE_IN_BYTES : I
public static final TYPE : Lnet/minecraft/nbt/TagType;
private final list : Ljava/util/List;
public <init>()V
 <init>(Ljava/util/List;)V
private static tryUnwrap(Lnet/minecraft/nbt/CompoundTag;)Lnet/minecraft/nbt/Tag;
private static isWrapper(Lnet/minecraft/nbt/CompoundTag;)Z
private static wrapIfNeeded(BLnet/minecraft/nbt/Tag;)Lnet/minecraft/nbt/Tag;
private static wrapElement(Lnet/minecraft/nbt/Tag;)Lnet/minecraft/nbt/CompoundTag;
public write(Ljava/io/DataOutput;)V
 identifyRawElementType()B
public addAndUnwrap(Lnet/minecraft/nbt/Tag;)V
public sizeInBytes()I
public getId()B
public getType()Lnet/minecraft/nbt/TagType;
public toString()Ljava/lang/String;
public remove(I)Lnet/minecraft/nbt/Tag;
public isEmpty()Z
public getCompound(I)Ljava/util/Optional;
public getCompoundOrEmpty(I)Lnet/minecraft/nbt/CompoundTag;
public getList(I)Ljava/util/Optional;
public getListOrEmpty(I)Lnet/minecraft/nbt/ListTag;
public getShort(I)Ljava/util/Optional;
public getShortOr(IS)S
public getInt(I)Ljava/util/Optional;
public getIntOr(II)I
public getIntArray(I)Ljava/util/Optional;
public getLongArray(I)Ljava/util/Optional;
public getDouble(I)Ljava/util/Optional;
public getDoubleOr(ID)D
public getFloat(I)Ljava/util/Optional;
public getFloatOr(IF)F
public getString(I)Ljava/util/Optional;
public getStringOr(ILjava/lang/String;)Ljava/lang/String;
private getNullable(I)Lnet/minecraft/nbt/Tag;
private getOptional(I)Ljava/util/Optional;
public size()I
public get(I)Lnet/minecraft/nbt/Tag;
public set(ILnet/minecraft/nbt/Tag;)Lnet/minecraft/nbt/Tag;
public add(ILnet/minecraft/nbt/Tag;)V
public setTag(ILnet/minecraft/nbt/Tag;)Z
public addTag(ILnet/minecraft/nbt/Tag;)Z
public copy()Lnet/minecraft/nbt/ListTag;
public asList()Ljava/util/Optional;
public equals(Ljava/lang/Object;)Z
public hashCode()I
public stream()Ljava/util/stream/Stream;
public compoundStream()Ljava/util/stream/Stream;
public accept(Lnet/minecraft/nbt/TagVisitor;)V
public clear()V
public accept(Lnet/minecraft/nbt/StreamTagVisitor;)Lnet/minecraft/nbt/StreamTagVisitor$ValueResult;
public synthetic remove(I)Ljava/lang/Object;
public synthetic add(ILjava/lang/Object;)V
public synthetic set(ILjava/lang/Object;)Ljava/lang/Object;
public synthetic get(I)Ljava/lang/Object;
public synthetic copy()Lnet/minecraft/nbt/Tag;
private static synthetic lambda$compoundStream$0(Lnet/minecraft/nbt/Tag;Ljava/util/function/Consumer;)V
static <clinit>()V
```
