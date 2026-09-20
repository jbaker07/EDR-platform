---
type: "interface"
fqcn: "net.minecraft.world.entity.ai.attributes.AttributeSupplier$Builder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity.ai.attributes.AttributeSupplier$Builder

System: [[20-Systems/net.minecraft.world.entity|net.minecraft.world.entity]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `build` | `()Lnet/minecraft/world/entity/ai/attributes/AttributeSupplier;` | exact | invokevirtual@2 in `FabricDefaultAttributeRegistry.register` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `build` | `()Lnet/minecraft/world/entity/ai/attributes/AttributeSupplier;` | exact | invokevirtual@31 in `FabricDefaultAttributeRegistryImpl$ModifyContextImpl.lambda$modify$0` | unknown | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |

## Declared members (2 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final builder : Lcom/google/common/collect/ImmutableMap$Builder;
private instanceFrozen : Z
public <init>()V
private create(Lnet/minecraft/core/Holder;)Lnet/minecraft/world/entity/ai/attributes/AttributeInstance;
public add(Lnet/minecraft/core/Holder;)Lnet/minecraft/world/entity/ai/attributes/AttributeSupplier$Builder;
public add(Lnet/minecraft/core/Holder;D)Lnet/minecraft/world/entity/ai/attributes/AttributeSupplier$Builder;
public build()Lnet/minecraft/world/entity/ai/attributes/AttributeSupplier;
private synthetic lambda$create$0(Lnet/minecraft/core/Holder;Lnet/minecraft/world/entity/ai/attributes/AttributeInstance;)V
```
