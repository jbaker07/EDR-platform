---
type: "interface"
fqcn: "net.minecraft.client.resources.model.sprite.AtlasManager$AtlasConfig"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.resources.model.sprite.AtlasManager$AtlasConfig

System: [[20-Systems/net.minecraft.client.resources|net.minecraft.client.resources]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `definitionLocation` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@62 in `AtlasRegistryImpl.register` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `definitionLocation` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@78 in `AtlasRegistryImpl.register` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `definitionLocation` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@107 in `AtlasRegistryImpl.register` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `definitionLocation` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@75 in `AtlasRegistryImpl.<clinit>` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `textureId` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@27 in `AtlasRegistryImpl.register` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `textureId` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@43 in `AtlasRegistryImpl.register` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `textureId` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@120 in `AtlasRegistryImpl.register` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `textureId` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@62 in `AtlasRegistryImpl.<clinit>` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (4 fields, 9 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final textureId : Lnet/minecraft/resources/Identifier;
private final definitionLocation : Lnet/minecraft/resources/Identifier;
private final createMipmaps : Z
private final additionalMetadata : Ljava/util/Set;
public <init>(Lnet/minecraft/resources/Identifier;Lnet/minecraft/resources/Identifier;Z)V
public <init>(Lnet/minecraft/resources/Identifier;Lnet/minecraft/resources/Identifier;ZLjava/util/Set;)V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public textureId()Lnet/minecraft/resources/Identifier;
public definitionLocation()Lnet/minecraft/resources/Identifier;
public createMipmaps()Z
public additionalMetadata()Ljava/util/Set;
```
