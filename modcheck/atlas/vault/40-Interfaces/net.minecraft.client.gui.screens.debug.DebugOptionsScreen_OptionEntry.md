---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.debug.DebugOptionsScreen$OptionEntry"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.debug.DebugOptionsScreen$OptionEntry

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`class` ; extends `net/minecraft/client/gui/screens/debug/DebugOptionsScreen$AbstractOptionEntry`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| wraps | `<init>` | `(Lnet/minecraft/client/gui/screens/debug/DebugOptionsScreen;Lnet/minec` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (9 fields, 10 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final BUTTON_WIDTH : I
private final location : Lnet/minecraft/resources/Identifier;
protected final children : Ljava/util/List;
private final always : Lnet/minecraft/client/gui/components/CycleButton;
private final overlay : Lnet/minecraft/client/gui/components/CycleButton;
private final never : Lnet/minecraft/client/gui/components/CycleButton;
private final name : Ljava/lang/String;
private final isAllowed : Z
final synthetic this$0 : Lnet/minecraft/client/gui/screens/debug/DebugOptionsScreen;
public <init>(Lnet/minecraft/client/gui/screens/debug/DebugOptionsScreen;Lnet/minecraft/resources/Identifier;)V
private narrateButton(Lnet/minecraft/client/gui/components/CycleButton;)Lnet/minecraft/network/chat/MutableComponent;
private setValue(Lnet/minecraft/resources/Identifier;Lnet/minecraft/client/gui/components/debug/DebugScreenEntryStatus;)V
public children()Ljava/util/List;
public narratables()Ljava/util/List;
public extractContent(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIZF)V
public refreshEntry()V
private synthetic lambda$new$2(Lnet/minecraft/resources/Identifier;Lnet/minecraft/client/gui/components/CycleButton;Ljava/lang/Boolean;)V
private synthetic lambda$new$1(Lnet/minecraft/resources/Identifier;Lnet/minecraft/client/gui/components/CycleButton;Ljava/lang/Boolean;)V
private synthetic lambda$new$0(Lnet/minecraft/resources/Identifier;Lnet/minecraft/client/gui/components/CycleButton;Ljava/lang/Boolean;)V
```
