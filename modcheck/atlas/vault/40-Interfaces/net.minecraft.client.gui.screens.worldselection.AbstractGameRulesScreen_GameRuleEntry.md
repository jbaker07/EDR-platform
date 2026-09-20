---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.worldselection.AbstractGameRulesScreen$GameRuleEntry"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.worldselection.AbstractGameRulesScreen$GameRuleEntry

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/client/gui/screens/worldselection/AbstractGa` | `` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `"<init>"(Lnet/minecraft/client/gui/screens/worldselection/AbstractGa` | `` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |

## Declared members (7, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public abstract class net.minecraft.client.gui.screens.worldselection.AbstractGameRulesScreen$GameRuleEntry extends net.minecraft.client.gui.screens.worldselection.AbstractGameRulesScreen$RuleEntry {
    private final java.util.List<net.minecraft.util.FormattedCharSequence> label;
    protected final java.util.List<net.minecraft.client.gui.components.AbstractWidget> children;
    final net.minecraft.client.gui.screens.worldselection.AbstractGameRulesScreen this$0;
    public net.minecraft.client.gui.screens.worldselection.AbstractGameRulesScreen$GameRuleEntry(java.util.List<net.minecraft.util.FormattedCharSequence>, net.minecraft.network.chat.Component);
    public java.util.List<? extends net.minecraft.client.gui.components.events.GuiEventListener> children();
    public java.util.List<? extends net.minecraft.client.gui.narration.NarratableEntry> narratables();
    protected void extractLabel(net.minecraft.client.gui.GuiGraphicsExtractor, int, int);
}
```
