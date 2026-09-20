---
type: "interface"
fqcn: "net.minecraft.nbt.NbtIo"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.nbt.NbtIo

System: [[20-Systems/net.minecraft.nbt|net.minecraft.nbt]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `readCompressed` | `(Ljava/nio/file/Path;Lnet/minecraft/nbt/NbtAccounter;)Lnet/minecraft/n` | exact | invokestatic@19 in `RegistryCustomContentState.readFile` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `readCompressed` | `(Ljava/nio/file/Path;Lnet/minecraft/nbt/NbtAccounter;)Lnet/minecraft/n` | exact | invokestatic@142 in `DefaultResourcePackStorage.read` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `writeCompressed` | `(Lnet/minecraft/nbt/CompoundTag;Ljava/nio/file/Path;)V` | exact | invokestatic@58 in `CreateWorldScreenMixin.createLevelDataForServers` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `writeCompressed` | `(Lnet/minecraft/nbt/CompoundTag;Ljava/nio/file/Path;)V` | exact | invokestatic@35 in `RegistryCustomContentState.writeFile` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (1 fields, 22 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final SYNC_OUTPUT_OPTIONS : [Ljava/nio/file/OpenOption;
public <init>()V
public static readCompressed(Ljava/nio/file/Path;Lnet/minecraft/nbt/NbtAccounter;)Lnet/minecraft/nbt/CompoundTag;
private static createDecompressorStream(Ljava/io/InputStream;)Ljava/io/DataInputStream;
private static createCompressorStream(Ljava/io/OutputStream;)Ljava/io/DataOutputStream;
public static readCompressed(Ljava/io/InputStream;Lnet/minecraft/nbt/NbtAccounter;)Lnet/minecraft/nbt/CompoundTag;
public static parseCompressed(Ljava/nio/file/Path;Lnet/minecraft/nbt/StreamTagVisitor;Lnet/minecraft/nbt/NbtAccounter;)V
public static parseCompressed(Ljava/io/InputStream;Lnet/minecraft/nbt/StreamTagVisitor;Lnet/minecraft/nbt/NbtAccounter;)V
public static writeCompressed(Lnet/minecraft/nbt/CompoundTag;Ljava/nio/file/Path;)V
public static writeCompressed(Lnet/minecraft/nbt/CompoundTag;Ljava/io/OutputStream;)V
public static write(Lnet/minecraft/nbt/CompoundTag;Ljava/nio/file/Path;)V
public static read(Ljava/nio/file/Path;)Lnet/minecraft/nbt/CompoundTag;
public static read(Ljava/io/DataInput;)Lnet/minecraft/nbt/CompoundTag;
public static read(Ljava/io/DataInput;Lnet/minecraft/nbt/NbtAccounter;)Lnet/minecraft/nbt/CompoundTag;
public static write(Lnet/minecraft/nbt/CompoundTag;Ljava/io/DataOutput;)V
public static parse(Ljava/io/DataInput;Lnet/minecraft/nbt/StreamTagVisitor;Lnet/minecraft/nbt/NbtAccounter;)V
public static readAnyTag(Ljava/io/DataInput;Lnet/minecraft/nbt/NbtAccounter;)Lnet/minecraft/nbt/Tag;
public static writeAnyTag(Lnet/minecraft/nbt/Tag;Ljava/io/DataOutput;)V
public static writeUnnamedTag(Lnet/minecraft/nbt/Tag;Ljava/io/DataOutput;)V
public static writeUnnamedTagWithFallback(Lnet/minecraft/nbt/Tag;Ljava/io/DataOutput;)V
public static readUnnamedTag(Ljava/io/DataInput;Lnet/minecraft/nbt/NbtAccounter;)Lnet/minecraft/nbt/Tag;
private static readTagSafe(Ljava/io/DataInput;Lnet/minecraft/nbt/NbtAccounter;B)Lnet/minecraft/nbt/Tag;
static <clinit>()V
```
