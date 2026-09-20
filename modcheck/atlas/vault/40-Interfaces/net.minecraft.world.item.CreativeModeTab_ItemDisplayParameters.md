---
type: "interface"
fqcn: "net.minecraft.world.item.CreativeModeTab$ItemDisplayParameters"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.CreativeModeTab$ItemDisplayParameters

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `enabledFeatures` | `()Lnet/minecraft/world/flag/FeatureFlagSet;` | exact | invokevirtual@4 in `FabricCreativeModeTabOutput.getEnabledFeatures` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `hasPermissions` | `()Z` | exact | invokevirtual@4 in `FabricCreativeModeTabOutput.shouldShowOpRestrictedItems` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `hasPermissions` | `()Z` | exact | invokevirtual@113 in `CreativeModeTabMixin.getStacks` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `hasPermissions` | `()Z` | exact | invokevirtual@17 in `CreativeModeInventoryScreenMixin.hasAdditionalPages` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |

## Declared members (3 fields, 8 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final enabledFeatures : Lnet/minecraft/world/flag/FeatureFlagSet;
private final hasPermissions : Z
private final holders : Lnet/minecraft/core/HolderLookup$Provider;
public <init>(Lnet/minecraft/world/flag/FeatureFlagSet;ZLnet/minecraft/core/HolderLookup$Provider;)V
public needsUpdate(Lnet/minecraft/world/flag/FeatureFlagSet;ZLnet/minecraft/core/HolderLookup$Provider;)Z
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public enabledFeatures()Lnet/minecraft/world/flag/FeatureFlagSet;
public hasPermissions()Z
public holders()Lnet/minecraft/core/HolderLookup$Provider;
```
