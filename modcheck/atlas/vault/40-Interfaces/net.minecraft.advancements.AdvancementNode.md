---
type: "interface"
fqcn: "net.minecraft.advancements.AdvancementNode"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.advancements.AdvancementNode

System: [[20-Systems/net.minecraft.advancements|net.minecraft.advancements]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `holder` | `()Lnet/minecraft/advancements/AdvancementHolder;` | exact | invokevirtual@4 in `AdvancementWidgetMixin.extractAdvancementIcon` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `holder` | `()Lnet/minecraft/advancements/AdvancementHolder;` | exact | invokevirtual@53 in `AdvancementWidgetMixin.extractAdvancementIcon` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `holder` | `()Lnet/minecraft/advancements/AdvancementHolder;` | exact | invokevirtual@4 in `AdvancementWidgetMixin.extractAdvancementFrame` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `holder` | `()Lnet/minecraft/advancements/AdvancementHolder;` | exact | invokevirtual@53 in `AdvancementWidgetMixin.extractAdvancementFrame` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| calls | `holder` | `()Lnet/minecraft/advancements/AdvancementHolder;` | exact | invokevirtual@4 in `AdvancementWidgetMixin.captureExtractTooltip` | unknown | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (5 fields, 16 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final holder : Lnet/minecraft/advancements/AdvancementHolder;
private final parent : Lnet/minecraft/advancements/AdvancementNode;
private final children : Ljava/util/Set;
private x : F
private y : F
public <init>(Lnet/minecraft/advancements/AdvancementHolder;Lnet/minecraft/advancements/AdvancementNode;)V
public advancement()Lnet/minecraft/advancements/Advancement;
public holder()Lnet/minecraft/advancements/AdvancementHolder;
public isTask()Z
public isRoot()Z
public parent()Lnet/minecraft/advancements/AdvancementNode;
public root()Lnet/minecraft/advancements/AdvancementNode;
public static getRoot(Lnet/minecraft/advancements/AdvancementNode;)Lnet/minecraft/advancements/AdvancementNode;
public children()Ljava/lang/Iterable;
public addChild(Lnet/minecraft/advancements/AdvancementNode;)V
public setLocation(FF)V
public x()F
public y()F
public equals(Ljava/lang/Object;)Z
public hashCode()I
public toString()Ljava/lang/String;
```
