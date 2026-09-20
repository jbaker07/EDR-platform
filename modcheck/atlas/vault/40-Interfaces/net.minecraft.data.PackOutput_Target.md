---
type: "interface"
fqcn: "net.minecraft.data.PackOutput$Target"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.data.PackOutput$Target

System: [[20-Systems/net.minecraft.data|net.minecraft.data]]

`enum` public final; extends `java/lang/Enum`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `DATA_PACK` | `Lnet/minecraft/data/PackOutput$Target;` | exact | getstatic@85 in `FabricDynamicRegistryProvider.writeHolders` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `DATA_PACK` | `Lnet/minecraft/data/PackOutput$Target;` | exact | getstatic@2 in `TagsProviderMixin.initPathResolver` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `RESOURCE_PACK` | `Lnet/minecraft/data/PackOutput$Target;` | exact | getstatic@4 in `FabricSoundsProvider.lambda$run$3` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| reads | `RESOURCE_PACK` | `Lnet/minecraft/data/PackOutput$Target;` | exact | getstatic@4 in `FabricLanguageProvider.getLangFilePath` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (5 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final DATA_PACK : Lnet/minecraft/data/PackOutput$Target;
public static final RESOURCE_PACK : Lnet/minecraft/data/PackOutput$Target;
public static final REPORTS : Lnet/minecraft/data/PackOutput$Target;
private final directory : Ljava/lang/String;
private static final synthetic $VALUES : [Lnet/minecraft/data/PackOutput$Target;
public static values()[Lnet/minecraft/data/PackOutput$Target;
public static valueOf(Ljava/lang/String;)Lnet/minecraft/data/PackOutput$Target;
private <init>(Ljava/lang/String;ILjava/lang/String;)V
private static synthetic $values()[Lnet/minecraft/data/PackOutput$Target;
static <clinit>()V
```
