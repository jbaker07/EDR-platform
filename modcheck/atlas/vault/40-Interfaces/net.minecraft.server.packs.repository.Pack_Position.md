---
type: "interface"
fqcn: "net.minecraft.server.packs.repository.Pack$Position"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.packs.repository.Pack$Position

System: [[20-Systems/net.minecraft.server.packs|net.minecraft.server.packs]]

`enum` public final; extends `java/lang/Enum`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `TOP` | `Lnet/minecraft/server/packs/repository/Pack$Position;` | exact | getstatic@108 in `ResourceLoaderImpl.registerBuiltinResourcePacks` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `TOP` | `Lnet/minecraft/server/packs/repository/Pack$Position;` | exact | getstatic@23 in `ModResourcePackCreator.<init>` | unknown | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (3 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final TOP : Lnet/minecraft/server/packs/repository/Pack$Position;
public static final BOTTOM : Lnet/minecraft/server/packs/repository/Pack$Position;
private static final synthetic $VALUES : [Lnet/minecraft/server/packs/repository/Pack$Position;
public static values()[Lnet/minecraft/server/packs/repository/Pack$Position;
public static valueOf(Ljava/lang/String;)Lnet/minecraft/server/packs/repository/Pack$Position;
private <init>(Ljava/lang/String;I)V
public insert(Ljava/util/List;Ljava/lang/Object;Ljava/util/function/Function;Z)I
public opposite()Lnet/minecraft/server/packs/repository/Pack$Position;
private static synthetic $values()[Lnet/minecraft/server/packs/repository/Pack$Position;
static <clinit>()V
```
