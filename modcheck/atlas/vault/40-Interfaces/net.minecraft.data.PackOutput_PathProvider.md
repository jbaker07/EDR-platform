---
type: "interface"
fqcn: "net.minecraft.data.PackOutput$PathProvider"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.PackOutput$PathProvider

System: [[20-Systems/net.minecraft.data|net.minecraft.data]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `json(Lnet/minecraft/resources/Identifier;)Ljava/nio/file/Path;` | `` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `json(Lnet/minecraft/resources/Identifier;)Ljava/nio/file/Path;` | `` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (8, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.data.PackOutput$PathProvider {
    private final java.nio.file.Path root;
    private final java.lang.String kind;
    private net.minecraft.data.PackOutput$PathProvider(net.minecraft.data.PackOutput, net.minecraft.data.PackOutput$Target, java.lang.String);
    public java.nio.file.Path file(net.minecraft.resources.Identifier, java.lang.String);
    public java.nio.file.Path json(net.minecraft.resources.Identifier);
    public java.nio.file.Path json(net.minecraft.resources.ResourceKey<?>);
    private java.lang.String lambda$json$0(java.lang.String);
    private java.lang.String lambda$file$0(java.lang.String, java.lang.String);
}
```
