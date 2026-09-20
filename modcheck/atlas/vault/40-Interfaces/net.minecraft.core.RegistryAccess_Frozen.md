---
type: "interface"
fqcn: "net.minecraft.core.RegistryAccess$Frozen"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.core.RegistryAccess$Frozen

System: [[20-Systems/net.minecraft.core|net.minecraft.core]]

`interface` public abstract; extends `java/lang/Object`; implements `net/minecraft/core/RegistryAccess`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `lookupOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/Registry;` | inherited_exact | invokeinterface@14 in `ReloadableServerRegistriesMixin.lambda$modifyAdvancements$0` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `lookupOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/Registry;` | inherited_exact | invokeinterface@10 in `TestWorldBuilderImpl.setConsistentSettings` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `lookupOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/Registry;` | inherited_exact | invokeinterface@11 in `TranslationConventionLogWarnings.lambda$setupUntranslatedItemTagWar | unknown | [[30-Mechanisms/fabric-convention-tags-v2|fabric-convention-tags-v2]] | direct_reference |
| calls | `lookupOrThrow` | `(Lnet/minecraft/resources/ResourceKey;)Lnet/minecraft/core/Registry;` | inherited_exact | invokeinterface@4 in `ReloadableServerRegistriesMixin.lambda$modifyLootTables$0` | unknown | [[30-Mechanisms/fabric-loot-api-v3|fabric-loot-api-v3]] | direct_reference |
| calls | `registries` | `()Ljava/util/stream/Stream;` | inherited_exact | invokeinterface@5 in `TagAliasLoader.applyToDynamicRegistries` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |

## Declared members (0 fields, 0 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
```
