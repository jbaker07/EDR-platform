---
type: "interface"
fqcn: "net.minecraft.world.attribute.EnvironmentAttributeMap$Builder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.attribute.EnvironmentAttributeMap$Builder

System: [[20-Systems/net.minecraft.world.attribute|net.minecraft.world.attribute]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `build` | `()Lnet/minecraft/world/attribute/EnvironmentAttributeMap;` | exact | invokevirtual@2 in `BiomeModificationContext$AttributesContext.addAll` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `build` | `()Lnet/minecraft/world/attribute/EnvironmentAttributeMap;` | exact | invokevirtual@31 in `BiomeModificationContextImpl$AttributesContextImpl.addAllRaw` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `build` | `()Lnet/minecraft/world/attribute/EnvironmentAttributeMap;` | exact | invokevirtual@32 in `BiomeModificationContextImpl$AttributesContextImpl.setRaw` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `build` | `()Lnet/minecraft/world/attribute/EnvironmentAttributeMap;` | exact | invokevirtual@36 in `BiomeModificationContextImpl$AttributesContextImpl.setModifierRaw` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `build` | `()Lnet/minecraft/world/attribute/EnvironmentAttributeMap;` | exact | invokevirtual@39 in `DimensionModificationImpl.applyChanges` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `modify` | `(Lnet/minecraft/world/attribute/EnvironmentAttribute;Lnet/minecraft/wo` | exact | invokevirtual@23 in `BiomeModificationContextImpl$AttributesContextImpl.setModifierRaw` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `putAll` | `(Lnet/minecraft/world/attribute/EnvironmentAttributeMap;)Lnet/minecraf` | exact | invokevirtual@13 in `BiomeModificationContextImpl$AttributesContextImpl.addAllRaw` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `putAll` | `(Lnet/minecraft/world/attribute/EnvironmentAttributeMap;)Lnet/minecraf` | exact | invokevirtual@19 in `BiomeModificationContextImpl$AttributesContextImpl.addAllRaw` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `putAll` | `(Lnet/minecraft/world/attribute/EnvironmentAttributeMap;)Lnet/minecraf` | exact | invokevirtual@13 in `BiomeModificationContextImpl$AttributesContextImpl.setRaw` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `putAll` | `(Lnet/minecraft/world/attribute/EnvironmentAttributeMap;)Lnet/minecraf` | exact | invokevirtual@13 in `BiomeModificationContextImpl$AttributesContextImpl.setModifierRaw` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |
| calls | `putAll` | `(Lnet/minecraft/world/attribute/EnvironmentAttributeMap;)Lnet/minecraf` | exact | invokevirtual@17 in `DimensionModificationImpl.applyChanges` | unknown | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| calls | `set` | `(Lnet/minecraft/world/attribute/EnvironmentAttribute;Ljava/lang/Object` | exact | invokevirtual@20 in `BiomeModificationContextImpl$AttributesContextImpl.setRaw` | unknown | [[30-Mechanisms/fabric-biome-api-v1|fabric-biome-api-v1]] | direct_reference |

## Declared members (1 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final entries : Ljava/util/Map;
private <init>()V
public putAll(Lnet/minecraft/world/attribute/EnvironmentAttributeMap;)Lnet/minecraft/world/attribute/EnvironmentAttributeMap$Builder;
public modify(Lnet/minecraft/world/attribute/EnvironmentAttribute;Lnet/minecraft/world/attribute/modifier/AttributeModifier;Ljava/lang/Object;)Lnet/minecraft/world/attribute/EnvironmentAttributeMap$Builder;
public set(Lnet/minecraft/world/attribute/EnvironmentAttribute;Ljava/lang/Object;)Lnet/minecraft/world/attribute/EnvironmentAttributeMap$Builder;
public build()Lnet/minecraft/world/attribute/EnvironmentAttributeMap;
```
