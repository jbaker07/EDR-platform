---
type: "interface"
fqcn: "net.minecraft.data.PackOutput"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.PackOutput

System: [[20-Systems/net.minecraft.data|net.minecraft.data]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `createPathProvider(Lnet/minecraft/data/PackOutput$Target;Ljava/lang/String;)Ln` | `` | both | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (8, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.data.PackOutput {
    private final java.nio.file.Path outputFolder;
    public net.minecraft.data.PackOutput(java.nio.file.Path);
    public java.nio.file.Path getOutputFolder();
    public java.nio.file.Path getOutputFolder(net.minecraft.data.PackOutput$Target);
    public net.minecraft.data.PackOutput$PathProvider createPathProvider(net.minecraft.data.PackOutput$Target, java.lang.String);
    public net.minecraft.data.PackOutput$PathProvider createRegistryElementsPathProvider(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<?>>);
    public net.minecraft.data.PackOutput$PathProvider createRegistryTagsPathProvider(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<?>>);
    public net.minecraft.data.PackOutput$PathProvider createRegistryComponentPathProvider(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<?>>);
}
```
