---
type: "interface"
fqcn: "net.minecraft.world.entity.ai.attributes.AttributeSupplier"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity.ai.attributes.AttributeSupplier

System: [[20-Systems/net.minecraft.world.entity|net.minecraft.world.entity]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `builder` | `()Lnet/minecraft/world/entity/ai/attributes/AttributeSupplier$Builder;` | exact | invokestatic@0 in `FabricDefaultAttributeRegistryImpl.createFromExistingSupplier` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |

## Declared members (1 fields, 9 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final instances : Ljava/util/Map;
private <init>(Ljava/util/Map;)V
private getAttributeInstance(Lnet/minecraft/core/Holder;)Lnet/minecraft/world/entity/ai/attributes/AttributeInstance;
public getValue(Lnet/minecraft/core/Holder;)D
public getBaseValue(Lnet/minecraft/core/Holder;)D
public getModifierValue(Lnet/minecraft/core/Holder;Lnet/minecraft/resources/Identifier;)D
public createInstance(Ljava/util/function/Consumer;Lnet/minecraft/core/Holder;)Lnet/minecraft/world/entity/ai/attributes/AttributeInstance;
public static builder()Lnet/minecraft/world/entity/ai/attributes/AttributeSupplier$Builder;
public hasAttribute(Lnet/minecraft/core/Holder;)Z
public hasModifier(Lnet/minecraft/core/Holder;Lnet/minecraft/resources/Identifier;)Z
```
