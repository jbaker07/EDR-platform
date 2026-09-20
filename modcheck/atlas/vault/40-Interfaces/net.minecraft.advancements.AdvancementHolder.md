---
type: "interface"
fqcn: "net.minecraft.advancements.AdvancementHolder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.advancements.AdvancementHolder

System: [[20-Systems/net.minecraft.advancements|net.minecraft.advancements]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@5 in `FabricAdvancementProvider.getOutputPath` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@80 in `FabricAdvancementProvider.lambda$run$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@97 in `FabricAdvancementProvider.lambda$run$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@10 in `AdvancementTabMixin.preBackgroundRender` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@33 in `AdvancementTabTypeMixin.extractAdvancementIcon` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@4 in `AdvancementToastMixin.extractAdvancementIcon` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@7 in `AdvancementWidgetMixin.extractAdvancementIcon` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@7 in `AdvancementWidgetMixin.extractAdvancementFrame` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@7 in `AdvancementWidgetMixin.captureExtractTooltip` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `id` | `()Lnet/minecraft/resources/Identifier;` | exact | invokevirtual@8 in `AdvancementsScreenMixin.wrapDrawIcon` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `value` | `()Lnet/minecraft/advancements/Advancement;` | exact | invokevirtual@63 in `AdvancementUtil.modifyAdvancement` | unknown | [[30-Mechanisms/fabric-advancement-api-v1|fabric-advancement-api-v1]] | direct_reference |
| calls | `value` | `()Lnet/minecraft/advancements/Advancement;` | exact | invokevirtual@119 in `FabricAdvancementProvider.lambda$run$0` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `value` | `()Lnet/minecraft/advancements/Advancement;` | exact | invokevirtual@13 in `FabricRecipeProvider$1.accept` | unknown | [[30-Mechanisms/fabric-data-generation-api-v1|fabric-data-generation-api-v1]] | direct_reference |
| calls | `value` | `()Lnet/minecraft/advancements/Advancement;` | exact | invokevirtual@6 in `AdvancementRenderContext.advancement` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (3 fields, 8 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final id : Lnet/minecraft/resources/Identifier;
private final value : Lnet/minecraft/advancements/Advancement;
public static final STREAM_CODEC : Lnet/minecraft/network/codec/StreamCodec;
public <init>(Lnet/minecraft/resources/Identifier;Lnet/minecraft/advancements/Advancement;)V
public register(Lnet/minecraft/data/worldgen/BootstrapContext;)V
public equals(Ljava/lang/Object;)Z
public hashCode()I
public toString()Ljava/lang/String;
public id()Lnet/minecraft/resources/Identifier;
public value()Lnet/minecraft/advancements/Advancement;
static <clinit>()V
```
