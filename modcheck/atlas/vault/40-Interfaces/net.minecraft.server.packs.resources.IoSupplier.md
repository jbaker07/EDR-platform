---
type: "interface"
fqcn: "net.minecraft.server.packs.resources.IoSupplier"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.resources.IoSupplier

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `create(Ljava/nio/file/Path;)Lnet/minecraft/server/packs/resources/` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `get()Ljava/lang/Object;` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (5, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.server.packs.resources.IoSupplier<T> {
    public static net.minecraft.server.packs.resources.IoSupplier<java.io.InputStream> create(java.nio.file.Path);
    public static net.minecraft.server.packs.resources.IoSupplier<java.io.InputStream> create(java.util.zip.ZipFile, java.util.zip.ZipEntry);
    public abstract T get() throws java.io.IOException;
    private static java.io.InputStream lambda$create$1(java.util.zip.ZipFile, java.util.zip.ZipEntry) throws java.io.IOException;
    private static java.io.InputStream lambda$create$0(java.nio.file.Path) throws java.io.IOException;
}
```
