---
type: "interface"
fqcn: "net.minecraft.client.gui.components.Button$Builder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.components.Button$Builder

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `bounds` | `(IIII)Lnet/minecraft/client/gui/components/Button$Builder;` | exact | invokevirtual@81 in `EnumRuleEntry.<init>` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `bounds` | `(IIII)Lnet/minecraft/client/gui/components/Button$Builder;` | exact | invokevirtual@61 in `DetailedBackupConfirmScreen.init` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `build` | `()Lnet/minecraft/client/gui/components/Button;` | exact | invokevirtual@84 in `EnumRuleEntry.<init>` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `build` | `()Lnet/minecraft/client/gui/components/Button;` | exact | invokevirtual@64 in `DetailedBackupConfirmScreen.init` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `build` | `()Lnet/minecraft/client/gui/components/Button;` | exact | invokevirtual@22 in `DetailsScreen.addFooter` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `width` | `(I)Lnet/minecraft/client/gui/components/Button$Builder;` | exact | invokevirtual@19 in `DetailsScreen.addFooter` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (8 fields, 8 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final message : Lnet/minecraft/network/chat/Component;
private final onPress : Lnet/minecraft/client/gui/components/Button$OnPress;
private tooltip : Lnet/minecraft/client/gui/components/Tooltip;
private x : I
private y : I
private width : I
private height : I
private createNarration : Lnet/minecraft/client/gui/components/Button$CreateNarration;
public <init>(Lnet/minecraft/network/chat/Component;Lnet/minecraft/client/gui/components/Button$OnPress;)V
public pos(II)Lnet/minecraft/client/gui/components/Button$Builder;
public width(I)Lnet/minecraft/client/gui/components/Button$Builder;
public size(II)Lnet/minecraft/client/gui/components/Button$Builder;
public bounds(IIII)Lnet/minecraft/client/gui/components/Button$Builder;
public tooltip(Lnet/minecraft/client/gui/components/Tooltip;)Lnet/minecraft/client/gui/components/Button$Builder;
public createNarration(Lnet/minecraft/client/gui/components/Button$CreateNarration;)Lnet/minecraft/client/gui/components/Button$Builder;
public build()Lnet/minecraft/client/gui/components/Button;
```
