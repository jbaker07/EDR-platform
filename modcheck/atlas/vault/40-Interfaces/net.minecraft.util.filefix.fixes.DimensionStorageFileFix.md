---
type: "interface"
fqcn: "net.minecraft.util.filefix.fixes.DimensionStorageFileFix"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.filefix.fixes.DimensionStorageFileFix

System: [[20-Systems/net.minecraft.util.filefix|net.minecraft.util.filefix]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `makeFixer` | `@ModifyArg at INVOKE Lnet/minecraft/util/filefix/operations/FileFixOperations;ap` | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |

## Declared members (2, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.util.filefix.fixes.DimensionStorageFileFix extends net.minecraft.util.filefix.FileFix {
    public net.minecraft.util.filefix.fixes.DimensionStorageFileFix(com.mojang.datafixers.schemas.Schema);
    public void makeFixer();
}
```
