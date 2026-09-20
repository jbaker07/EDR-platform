---
type: "interface"
fqcn: "net.minecraft.world.item.crafting.display.SlotDisplay$ItemStackSlotDisplay"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.item.crafting.display.SlotDisplay$ItemStackSlotDisplay

System: [[20-Systems/net.minecraft.world.item|net.minecraft.world.item]]

`record` public final; extends `java/lang/Record`; implements `net/minecraft/world/item/crafting/display/SlotDisplay`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/world/item/ItemStackTemplate;)V` | exact | invokespecial@17 in `ComponentsIngredient.createEntryDisplay` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/world/item/ItemStackTemplate;)V` | exact | invokespecial@34 in `CustomDataIngredient.createEntryDisplay` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |

## Declared members (4 fields, 10 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final stack : Lnet/minecraft/world/item/ItemStackTemplate;
public static final MAP_CODEC : Lcom/mojang/serialization/MapCodec;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public static final TYPE : Lnet/minecraft/world/item/crafting/display/SlotDisplay$Type;
public <init>(Lnet/minecraft/world/item/ItemStackTemplate;)V
public type()Lnet/minecraft/world/item/crafting/display/SlotDisplay$Type;
public resolve(Lnet/minecraft/util/context/ContextMap;Lnet/minecraft/world/item/crafting/display/DisplayContentsFactory;)Ljava/util/stream/Stream;
public isEnabled(Lnet/minecraft/world/flag/FeatureFlagSet;)Z
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public stack()Lnet/minecraft/world/item/ItemStackTemplate;
private static synthetic lambda$static$0(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
static <clinit>()V
```
