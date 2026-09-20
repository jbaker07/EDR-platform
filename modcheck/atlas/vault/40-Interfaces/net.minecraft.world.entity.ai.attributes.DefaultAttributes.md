---
type: "interface"
fqcn: "net.minecraft.world.entity.ai.attributes.DefaultAttributes"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity.ai.attributes.DefaultAttributes

System: [[20-Systems/net.minecraft.world.entity|net.minecraft.world.entity]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `<clinit>` | `?` | selector_unsupported | @Inject at ['TAIL'] | both | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| reads | `SUPPLIERS` | `Ljava/util/Map;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | declared |

## Declared members (2 fields, 8 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final SUPPLIERS : Ljava/util/Map;
public <init>()V
public static getSupplier(Lnet/minecraft/world/entity/EntityType;)Lnet/minecraft/world/entity/ai/attributes/AttributeSupplier;
public static hasSupplier(Lnet/minecraft/world/entity/EntityType;)Z
public static validate()V
private static synthetic lambda$validate$2(Lnet/minecraft/resources/Identifier;)V
private static synthetic lambda$validate$1(Lnet/minecraft/world/entity/EntityType;)Z
private static synthetic lambda$validate$0(Lnet/minecraft/world/entity/EntityType;)Z
static <clinit>()V
```
