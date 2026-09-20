---
type: "interface"
fqcn: "net.minecraft.world.level.validation.DirectoryValidator"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.validation.DirectoryValidator

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Ljava/nio/file/PathMatcher;)V` | `` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (6, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.level.validation.DirectoryValidator {
    private final java.nio.file.PathMatcher symlinkTargetAllowList;
    public net.minecraft.world.level.validation.DirectoryValidator(java.nio.file.PathMatcher);
    public void validateSymlink(java.nio.file.Path, java.util.List<net.minecraft.world.level.validation.ForbiddenSymlinkInfo>) throws java.io.IOException;
    public java.util.List<net.minecraft.world.level.validation.ForbiddenSymlinkInfo> validateSymlink(java.nio.file.Path) throws java.io.IOException;
    public java.util.List<net.minecraft.world.level.validation.ForbiddenSymlinkInfo> validateDirectory(java.nio.file.Path, boolean) throws java.io.IOException;
    public void validateKnownDirectory(java.nio.file.Path, java.util.List<net.minecraft.world.level.validation.ForbiddenSymlinkInfo>) throws java.io.IOException;
}
```
