---
type: "interface"
fqcn: "net.minecraft.util.filefix.operations.FileFixOperations"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.filefix.operations.FileFixOperations

System: [[20-Systems/net.minecraft.util.filefix|net.minecraft.util.filefix]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `move(Ljava/lang/String;Ljava/lang/String;)Lnet/minecraft/util/fi` | `` | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |

## Declared members (7, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.util.filefix.operations.FileFixOperations {
    public net.minecraft.util.filefix.operations.FileFixOperations();
    public static net.minecraft.util.filefix.operations.Move moveSimple(java.lang.String);
    public static net.minecraft.util.filefix.operations.Move move(java.lang.String, java.lang.String);
    public static net.minecraft.util.filefix.operations.RegexMove moveRegex(java.lang.String, java.lang.String);
    public static net.minecraft.util.filefix.operations.DeleteFileOrEmptyDirectory delete(java.lang.String);
    public static net.minecraft.util.filefix.operations.ApplyInFolders applyInFolders(net.minecraft.util.filefix.access.FileRelation, java.util.List<net.minecraft.util.filefix.operations.FileFixOperation>);
    public static net.minecraft.util.filefix.operations.GroupMove groupMove(java.util.Map<java.lang.String, java.lang.String>, java.util.List<net.minecraft.util.filefix.operations.Move>);
}
```
