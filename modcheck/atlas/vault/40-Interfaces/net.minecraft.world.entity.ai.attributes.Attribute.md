---
type: "interface"
fqcn: "net.minecraft.world.entity.ai.attributes.Attribute"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity.ai.attributes.Attribute

System: [[20-Systems/net.minecraft.world.entity|net.minecraft.world.entity]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getDescriptionId` | `()Ljava/lang/String;` | exact | invokevirtual@10 in `FabricLanguageProvider$TranslationBuilder.addAttribute` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |

## Declared members (6 fields, 9 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final CODEC : Lcom/mojang/serialization/Codec;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
private final defaultValue : D
private syncable : Z
private final descriptionId : Ljava/lang/String;
private sentiment : Lnet/minecraft/world/entity/ai/attributes/Attribute$Sentiment;
protected <init>(Ljava/lang/String;D)V
public getDefaultValue()D
public isClientSyncable()Z
public setSyncable(Z)Lnet/minecraft/world/entity/ai/attributes/Attribute;
public setSentiment(Lnet/minecraft/world/entity/ai/attributes/Attribute$Sentiment;)Lnet/minecraft/world/entity/ai/attributes/Attribute;
public sanitizeValue(D)D
public getDescriptionId()Ljava/lang/String;
public getStyle(Z)Lnet/minecraft/ChatFormatting;
static <clinit>()V
```
