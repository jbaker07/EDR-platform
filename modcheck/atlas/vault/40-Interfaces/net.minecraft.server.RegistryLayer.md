---
type: "interface"
fqcn: "net.minecraft.server.RegistryLayer"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.RegistryLayer

System: [[20-Systems/net.minecraft.server|net.minecraft.server]]

`enum` public final; extends `java/lang/Enum`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `RELOADABLE` | `Lnet/minecraft/server/RegistryLayer;` | exact | getstatic@14 in `ReloadableServerResourcesMixin.applyDynamicTagAliases` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| reads | `WORLD` | `Lnet/minecraft/server/RegistryLayer;` | exact | getstatic@4 in `ReloadableServerResourcesMixin.applyDynamicTagAliases` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |

## Declared members (7 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final STATIC : Lnet/minecraft/server/RegistryLayer;
public static final WORLD : Lnet/minecraft/server/RegistryLayer;
public static final DIMENSIONS : Lnet/minecraft/server/RegistryLayer;
public static final RELOADABLE : Lnet/minecraft/server/RegistryLayer;
private static final VALUES : Ljava/util/List;
private static final STATIC_ACCESS : Lnet/minecraft/core/RegistryAccess$Frozen;
private static final synthetic $VALUES : [Lnet/minecraft/server/RegistryLayer;
public static values()[Lnet/minecraft/server/RegistryLayer;
public static valueOf(Ljava/lang/String;)Lnet/minecraft/server/RegistryLayer;
private <init>(Ljava/lang/String;I)V
public static createRegistryAccess()Lnet/minecraft/core/LayeredRegistryAccess;
private static synthetic $values()[Lnet/minecraft/server/RegistryLayer;
static <clinit>()V
```
