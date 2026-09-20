---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.debug.DebugOptionsScreen$OptionList"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.debug.DebugOptionsScreen$OptionList

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`class` public; extends `net/minecraft/client/gui/components/ContainerObjectSelectionList`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| wraps | `lambda$static$0` | `(Ljava/util/Map$Entry;Ljava/util/Map$Entry;)I` | name_only | @Redirect at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |
| wraps | `updateSearch` | `(Ljava/lang/String;)V` | name_only | @WrapOperation at ['INVOKE'] | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (3 fields, 8 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final COMPARATOR : Ljava/util/Comparator;
private static final ITEM_HEIGHT : I
final synthetic this$0 : Lnet/minecraft/client/gui/screens/debug/DebugOptionsScreen;
public <init>(Lnet/minecraft/client/gui/screens/debug/DebugOptionsScreen;)V
public extractWidgetRenderState(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V
public getRowWidth()I
public refreshEntries()V
public updateSearch(Ljava/lang/String;)V
private notifyListUpdated()V
private static synthetic lambda$static$0(Ljava/util/Map$Entry;Ljava/util/Map$Entry;)I
static <clinit>()V
```
