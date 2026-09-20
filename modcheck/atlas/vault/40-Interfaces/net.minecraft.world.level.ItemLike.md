---
type: "interface"
fqcn: "net.minecraft.world.level.ItemLike"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.level.ItemLike

System: [[20-Systems/net.minecraft.world.level|net.minecraft.world.level]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `asItem` | `()Lnet/minecraft/world/item/Item;` | exact | invokeinterface@22 in `ItemApiLookupImpl.registerSelf` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |
| calls | `asItem` | `()Lnet/minecraft/world/item/Item;` | exact | invokeinterface@46 in `ItemApiLookupImpl.registerForItems` | unknown | [[30-Mechanisms/fabric-api-lookup-api-v1|fabric-api-lookup-api-v1]] | direct_reference |
| calls | `asItem` | `()Lnet/minecraft/world/item/Item;` | exact | invokeinterface@1 in `VillagerInteractionRegistries.registerGatherableItem` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `asItem` | `()Lnet/minecraft/world/item/Item;` | exact | invokeinterface@16 in `VillagerInteractionRegistries.registerGatherableItem` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `asItem` | `()Lnet/minecraft/world/item/Item;` | exact | invokeinterface@1 in `VillagerInteractionRegistries.registerCompostable` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `asItem` | `()Lnet/minecraft/world/item/Item;` | exact | invokeinterface@16 in `VillagerInteractionRegistries.registerCompostable` | unknown | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |
| calls | `asItem` | `()Lnet/minecraft/world/item/Item;` | exact | invokeinterface@5 in `FabricCreativeModeTabOutput.insertBefore` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `asItem` | `()Lnet/minecraft/world/item/Item;` | exact | invokeinterface@5 in `FabricCreativeModeTabOutput.insertAfter` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `asItem` | `()Lnet/minecraft/world/item/Item;` | exact | invokeinterface@13 in `RecipeProviderMixin.adjustIdStonecutter` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `asItem` | `()Lnet/minecraft/world/item/Item;` | exact | invokeinterface@44 in `ArmorRendererRegistryImpl.register` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `asItem` | `()Lnet/minecraft/world/item/Item;` | exact | invokeinterface@60 in `ArmorRendererRegistryImpl.register` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `asItem` | `()Lnet/minecraft/world/item/Item;` | exact | invokeinterface@81 in `ArmorRendererRegistryImpl.register` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `asItem` | `()Lnet/minecraft/world/item/Item;` | exact | invokeinterface@1 in `ItemVariant.of` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (0 fields, 1 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract asItem()Lnet/minecraft/world/item/Item;
```
