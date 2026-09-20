---
type: "interface"
fqcn: "net.minecraft.world.level.validation.DirectoryValidator"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.validation.DirectoryValidator

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Ljava/nio/file/PathMatcher;)V` | exact | invokespecial@23 in `ModPackResourcesUtil.createModdedRepository` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (1 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final symlinkTargetAllowList : Ljava/nio/file/PathMatcher;
public <init>(Ljava/nio/file/PathMatcher;)V
public validateSymlink(Ljava/nio/file/Path;Ljava/util/List;)V
public validateSymlink(Ljava/nio/file/Path;)Ljava/util/List;
public validateDirectory(Ljava/nio/file/Path;Z)Ljava/util/List;
public validateKnownDirectory(Ljava/nio/file/Path;Ljava/util/List;)V
```
