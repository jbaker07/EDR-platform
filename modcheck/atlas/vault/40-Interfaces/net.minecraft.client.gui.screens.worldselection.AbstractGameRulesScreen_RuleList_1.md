---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.worldselection.AbstractGameRulesScreen$RuleList$1"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.worldselection.AbstractGameRulesScreen$RuleList$1

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`class` ; extends `java/lang/Object`; implements `net/minecraft/world/level/gamerules/GameRuleTypeVisitor`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `addEntry` | `(Lnet/minecraft/world/level/gamerules/GameRule;Lnet/minecraft/client/g` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | declared |
| reads | `this$1` | `Lnet/minecraft/client/gui/screens/worldselection/AbstractGameRulesScre` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | declared |
| wraps | `addEntry` | `(Lnet/minecraft/world/level/gamerules/GameRule;Lnet/minecraft/client/g` | exact | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |

## Declared members (3 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
final synthetic val$lowerCaseFilter : Ljava/lang/String;
final synthetic val$entries : Ljava/util/Map;
final synthetic this$1 : Lnet/minecraft/client/gui/screens/worldselection/AbstractGameRulesScreen$RuleList;
 <init>(Lnet/minecraft/client/gui/screens/worldselection/AbstractGameRulesScreen$RuleList;Ljava/lang/String;Ljava/util/Map;)V
public visitBoolean(Lnet/minecraft/world/level/gamerules/GameRule;)V
public visitInteger(Lnet/minecraft/world/level/gamerules/GameRule;)V
private addEntry(Lnet/minecraft/world/level/gamerules/GameRule;Lnet/minecraft/client/gui/screens/worldselection/AbstractGameRulesScreen$EntryFactory;)V
private static synthetic lambda$addEntry$0(Lnet/minecraft/world/level/gamerules/GameRuleCategory;)Ljava/util/Map;
private synthetic lambda$visitInteger$0(Lnet/minecraft/network/chat/Component;Ljava/util/List;Ljava/lang/String;Lnet/minecraft/world/level/gamerules/GameRule;)Lnet/minecraft/client/gui/screens/worldselection/AbstractGameRulesScreen$RuleEntry;
private synthetic lambda$visitBoolean$0(Lnet/minecraft/network/chat/Component;Ljava/util/List;Ljava/lang/String;Lnet/minecraft/world/level/gamerules/GameRule;)Lnet/minecraft/client/gui/screens/worldselection/AbstractGameRulesScreen$RuleEntry;
```
