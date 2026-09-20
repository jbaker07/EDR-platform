---
type: "interface"
fqcn: "net.minecraft.server.packs.PackLocationInfo"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.PackLocationInfo

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Ljava/lang/String;Lnet/minecraft/network/chat/Component;Lnet/minecraf` | exact | invokespecial@83 in `ResourceLoaderImpl.registerBuiltinResourcePacks` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `<init>` | `(Ljava/lang/String;Lnet/minecraft/network/chat/Component;Lnet/minecraf` | exact | invokespecial@288 in `ModNioPackResources.create` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `knownPackInfo` | `()Ljava/util/Optional;` | exact | invokevirtual@6 in `MinecraftServerMixin.lambda$init$0` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| calls | `source` | `()Lnet/minecraft/server/packs/repository/PackSource;` | exact | invokevirtual@5 in `PackMixin.lambda$onCreateResourcePack$0` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (4 fields, 10 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final id : Ljava/lang/String;
private final title : Lnet/minecraft/network/chat/Component;
private final source : Lnet/minecraft/server/packs/repository/PackSource;
private final knownPackInfo : Ljava/util/Optional;
public <init>(Ljava/lang/String;Lnet/minecraft/network/chat/Component;Lnet/minecraft/server/packs/repository/PackSource;Ljava/util/Optional;)V
public createChatLink(ZLnet/minecraft/network/chat/Component;)Lnet/minecraft/network/chat/Component;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public id()Ljava/lang/String;
public title()Lnet/minecraft/network/chat/Component;
public source()Lnet/minecraft/server/packs/repository/PackSource;
public knownPackInfo()Ljava/util/Optional;
private synthetic lambda$createChatLink$0(ZLnet/minecraft/network/chat/Component;Lnet/minecraft/network/chat/Style;)Lnet/minecraft/network/chat/Style;
```
