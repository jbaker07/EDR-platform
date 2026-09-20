---
type: "interface"
fqcn: "net.minecraft.world.attribute.modifier.MobSpawnSettingsModifier"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.attribute.modifier.MobSpawnSettingsModifier

System: [[20-Systems/net.minecraft.world.attribute|net.minecraft.world.attribute]]

`interface` public abstract; extends `java/lang/Object`; implements `net/minecraft/world/attribute/modifier/AttributeModifier`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `overlay` | `()Lnet/minecraft/world/attribute/modifier/MobSpawnSettingsModifier;` | exact | invokestatic@26 in `BiomeModificationContextImpl$SpawnSettingsContextImpl.reload` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `overlay` | `()Lnet/minecraft/world/attribute/modifier/MobSpawnSettingsModifier;` | exact | invokestatic@95 in `BiomeModificationContextImpl$SpawnSettingsContextImpl.applyPendingChan | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (0 fields, 3 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static overlay()Lnet/minecraft/world/attribute/modifier/MobSpawnSettingsModifier;
public argumentCodec(Lnet/minecraft/world/attribute/EnvironmentAttribute;)Lcom/mojang/serialization/Codec;
public argumentKeyframeLerp(Lnet/minecraft/world/attribute/EnvironmentAttribute;)Lnet/minecraft/world/attribute/LerpFunction;
```
