---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.worldselection.AbstractGameRulesScreen$GameRuleEntry"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.worldselection.AbstractGameRulesScreen$GameRuleEntry

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`abstract_class` public abstract; extends `net/minecraft/client/gui/screens/worldselection/AbstractGameRulesScreen$RuleEntry`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/client/gui/screens/worldselection/AbstractGameRulesScr` | exact | invokespecial@9 in `DoubleRuleEntry.<init>` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/client/gui/screens/worldselection/AbstractGameRulesScr` | exact | invokespecial@9 in `EnumRuleEntry.<init>` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |

## Declared members (3 fields, 4 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final label : Ljava/util/List;
protected final children : Ljava/util/List;
final synthetic this$0 : Lnet/minecraft/client/gui/screens/worldselection/AbstractGameRulesScreen;
public <init>(Lnet/minecraft/client/gui/screens/worldselection/AbstractGameRulesScreen;Ljava/util/List;Lnet/minecraft/network/chat/Component;)V
public children()Ljava/util/List;
public narratables()Ljava/util/List;
protected extractLabel(Lnet/minecraft/client/gui/GuiGraphicsExtractor;II)V
```
