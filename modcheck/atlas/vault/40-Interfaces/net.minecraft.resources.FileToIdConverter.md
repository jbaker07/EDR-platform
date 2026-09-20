---
type: "interface"
fqcn: "net.minecraft.resources.FileToIdConverter"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.resources.FileToIdConverter

System: [[20-Systems/net.minecraft.resources|net.minecraft.resources]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Ljava/lang/String;Ljava/lang/String;)V` | `` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| calls | `fileToId(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/resourc` | `` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `json(Ljava/lang/String;)Lnet/minecraft/resources/FileToIdConvert` | `` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `listMatchingResources(Lnet/minecraft/server/packs/resources/ResourceManager;)Ljav` | `` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `prefix()Ljava/lang/String;` | `` | both | [[30-Mechanisms/fabric-resource-conditions-api-v1|fabric-resource-conditions-api-v1]] | direct_reference |

## Declared members (18, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.resources.FileToIdConverter extends java.lang.Record {
    private final java.lang.String prefix;
    private final java.lang.String extension;
    public net.minecraft.resources.FileToIdConverter(java.lang.String, java.lang.String);
    public static net.minecraft.resources.FileToIdConverter json(java.lang.String);
    public static net.minecraft.resources.FileToIdConverter registry(net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<?>>);
    public net.minecraft.resources.Identifier idToFile(net.minecraft.resources.Identifier);
    public net.minecraft.resources.Identifier fileToId(net.minecraft.resources.Identifier);
    public boolean extensionMatches(net.minecraft.resources.Identifier);
    public boolean prefixMatches(net.minecraft.resources.Identifier);
    public boolean matches(net.minecraft.resources.Identifier);
    private net.minecraft.server.packs.resources.ResourceManager$Selector extensionSelector();
    public java.util.Map<net.minecraft.resources.Identifier, net.minecraft.server.packs.resources.Resource> listMatchingResources(net.minecraft.server.packs.resources.ResourceManager);
    public java.util.Map<net.minecraft.resources.Identifier, java.util.List<net.minecraft.server.packs.resources.Resource>> listMatchingResourceStacks(net.minecraft.server.packs.resources.ResourceManager);
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public java.lang.String prefix();
    public java.lang.String extension();
}
```
