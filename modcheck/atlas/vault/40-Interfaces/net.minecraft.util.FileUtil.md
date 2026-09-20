---
type: "interface"
fqcn: "net.minecraft.util.FileUtil"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.FileUtil

System: [[20-Systems/net.minecraft.util|net.minecraft.util]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `validatePath` | `([Ljava/lang/String;)V` | exact | invokestatic@1 in `ModNioPackResources.getRootResource` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (4 fields, 19 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final COPY_COUNTER_PATTERN : Ljava/util/regex/Pattern;
private static final MAX_FILE_NAME : I
private static final RESERVED_WINDOWS_FILENAMES : Ljava/util/regex/Pattern;
private static final STRICT_PATH_SEGMENT_CHECK : Ljava/util/regex/Pattern;
public <init>()V
public static sanitizeName(Ljava/lang/String;)Ljava/lang/String;
public static findAvailableName(Ljava/nio/file/Path;Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;
public static isPathPortable(Ljava/nio/file/Path;)Z
public static isPathPartPortable(Ljava/lang/String;)Z
public static getFullResourcePath(Ljava/lang/String;)Ljava/lang/String;
public static normalizeResourcePath(Ljava/lang/String;)Ljava/lang/String;
public static decomposePath(Ljava/lang/String;)Lcom/mojang/serialization/DataResult;
public static resolvePath(Ljava/nio/file/Path;Ljava/util/List;)Ljava/nio/file/Path;
private static containsAllowedCharactersOnly(Ljava/lang/String;)Z
public static isValidPathSegment(Ljava/lang/String;)Z
public static validatePath([Ljava/lang/String;)V
public static createDirectoriesSafe(Ljava/nio/file/Path;)V
public static isEmptyPath(Ljava/nio/file/Path;)Z
private static synthetic lambda$decomposePath$3(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;
private static synthetic lambda$decomposePath$2(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;
private static synthetic lambda$decomposePath$1(Ljava/lang/String;)Ljava/lang/String;
private static synthetic lambda$decomposePath$0(Ljava/lang/String;)Ljava/lang/String;
static <clinit>()V
```
