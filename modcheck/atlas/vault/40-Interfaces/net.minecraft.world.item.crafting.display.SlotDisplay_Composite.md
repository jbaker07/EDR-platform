---
type: "interface"
fqcn: "net.minecraft.world.item.crafting.display.SlotDisplay$Composite"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.crafting.display.SlotDisplay$Composite

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

`record` public final; extends `java/lang/Record`; implements `net/minecraft/world/item/crafting/display/SlotDisplay`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Ljava/util/List;)V` | exact | invokespecial@25 in `CustomIngredient.display` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `<init>` | `(Ljava/util/List;)V` | exact | invokespecial@28 in `CombinedIngredient.display` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `<init>` | `(Ljava/util/List;)V` | exact | invokespecial@27 in `ComponentsIngredient.display` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `<init>` | `(Ljava/util/List;)V` | exact | invokespecial@27 in `CustomDataIngredient.display` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |

## Declared members (4 fields, 12 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final contents : Ljava/util/List;
public static final MAP_CODEC : Lcom/mojang/serialization/MapCodec;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public static final TYPE : Lnet/minecraft/world/item/crafting/display/SlotDisplay$Type;
public <init>(Ljava/util/List;)V
public type()Lnet/minecraft/world/item/crafting/display/SlotDisplay$Type;
public resolve(Lnet/minecraft/util/context/ContextMap;Lnet/minecraft/world/item/crafting/display/DisplayContentsFactory;)Ljava/util/stream/Stream;
public isEnabled(Lnet/minecraft/world/flag/FeatureFlagSet;)Z
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public contents()Ljava/util/List;
private static synthetic lambda$isEnabled$0(Lnet/minecraft/world/flag/FeatureFlagSet;Lnet/minecraft/world/item/crafting/display/SlotDisplay;)Z
private static synthetic lambda$resolve$0(Lnet/minecraft/util/context/ContextMap;Lnet/minecraft/world/item/crafting/display/DisplayContentsFactory;Lnet/minecraft/world/item/crafting/display/SlotDisplay;)Ljava/util/stream/Stream;
private static synthetic lambda$static$0(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
static <clinit>()V
```
