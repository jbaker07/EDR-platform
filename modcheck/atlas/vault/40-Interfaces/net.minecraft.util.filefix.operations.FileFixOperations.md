---
type: "interface"
fqcn: "net.minecraft.util.filefix.operations.FileFixOperations"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.filefix.operations.FileFixOperations

System: [[20-Systems/net.minecraft.util.filefix|net.minecraft.util.filefix]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `move` | `(Ljava/lang/String;Ljava/lang/String;)Lnet/minecraft/util/filefix/oper` | exact | invokestatic@14 in `DimensionStorageFileFixMixin.addFabricAttachmentsMigration` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |

## Declared members (0 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public <init>()V
public static moveSimple(Ljava/lang/String;)Lnet/minecraft/util/filefix/operations/Move;
public static move(Ljava/lang/String;Ljava/lang/String;)Lnet/minecraft/util/filefix/operations/Move;
public static moveRegex(Ljava/lang/String;Ljava/lang/String;)Lnet/minecraft/util/filefix/operations/RegexMove;
public static delete(Ljava/lang/String;)Lnet/minecraft/util/filefix/operations/DeleteFileOrEmptyDirectory;
public static applyInFolders(Lnet/minecraft/util/filefix/access/FileRelation;Ljava/util/List;)Lnet/minecraft/util/filefix/operations/ApplyInFolders;
public static groupMove(Ljava/util/Map;Ljava/util/List;)Lnet/minecraft/util/filefix/operations/GroupMove;
```
