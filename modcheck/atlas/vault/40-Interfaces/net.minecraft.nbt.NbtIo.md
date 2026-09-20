---
type: "interface"
fqcn: "net.minecraft.nbt.NbtIo"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.nbt.NbtIo

System: [[20-Systems/net.minecraft.nbt|net.minecraft.nbt]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `readCompressed(Ljava/nio/file/Path;Lnet/minecraft/nbt/NbtAccounter;)Lnet/m` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `readCompressed(Ljava/nio/file/Path;Lnet/minecraft/nbt/NbtAccounter;)Lnet/m` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `writeCompressed(Lnet/minecraft/nbt/CompoundTag;Ljava/nio/file/Path;)V` | `` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `writeCompressed(Lnet/minecraft/nbt/CompoundTag;Ljava/nio/file/Path;)V` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (23, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.nbt.NbtIo {
    private static final java.nio.file.OpenOption[] SYNC_OUTPUT_OPTIONS;
    public net.minecraft.nbt.NbtIo();
    public static net.minecraft.nbt.CompoundTag readCompressed(java.nio.file.Path, net.minecraft.nbt.NbtAccounter) throws java.io.IOException;
    private static java.io.DataInputStream createDecompressorStream(java.io.InputStream) throws java.io.IOException;
    private static java.io.DataOutputStream createCompressorStream(java.io.OutputStream) throws java.io.IOException;
    public static net.minecraft.nbt.CompoundTag readCompressed(java.io.InputStream, net.minecraft.nbt.NbtAccounter) throws java.io.IOException;
    public static void parseCompressed(java.nio.file.Path, net.minecraft.nbt.StreamTagVisitor, net.minecraft.nbt.NbtAccounter) throws java.io.IOException;
    public static void parseCompressed(java.io.InputStream, net.minecraft.nbt.StreamTagVisitor, net.minecraft.nbt.NbtAccounter) throws java.io.IOException;
    public static void writeCompressed(net.minecraft.nbt.CompoundTag, java.nio.file.Path) throws java.io.IOException;
    public static void writeCompressed(net.minecraft.nbt.CompoundTag, java.io.OutputStream) throws java.io.IOException;
    public static void write(net.minecraft.nbt.CompoundTag, java.nio.file.Path) throws java.io.IOException;
    public static net.minecraft.nbt.CompoundTag read(java.nio.file.Path) throws java.io.IOException;
    public static net.minecraft.nbt.CompoundTag read(java.io.DataInput) throws java.io.IOException;
    public static net.minecraft.nbt.CompoundTag read(java.io.DataInput, net.minecraft.nbt.NbtAccounter) throws java.io.IOException;
    public static void write(net.minecraft.nbt.CompoundTag, java.io.DataOutput) throws java.io.IOException;
    public static void parse(java.io.DataInput, net.minecraft.nbt.StreamTagVisitor, net.minecraft.nbt.NbtAccounter) throws java.io.IOException;
    public static net.minecraft.nbt.Tag readAnyTag(java.io.DataInput, net.minecraft.nbt.NbtAccounter) throws java.io.IOException;
    public static void writeAnyTag(net.minecraft.nbt.Tag, java.io.DataOutput) throws java.io.IOException;
    public static void writeUnnamedTag(net.minecraft.nbt.Tag, java.io.DataOutput) throws java.io.IOException;
    public static void writeUnnamedTagWithFallback(net.minecraft.nbt.Tag, java.io.DataOutput) throws java.io.IOException;
    public static net.minecraft.nbt.Tag readUnnamedTag(java.io.DataInput, net.minecraft.nbt.NbtAccounter) throws java.io.IOException;
    private static net.minecraft.nbt.Tag readTagSafe(java.io.DataInput, net.minecraft.nbt.NbtAccounter, byte);
    static {};
}
```
