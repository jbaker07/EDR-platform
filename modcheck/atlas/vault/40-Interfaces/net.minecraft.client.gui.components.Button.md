---
type: "interface"
fqcn: "net.minecraft.client.gui.components.Button"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.components.Button

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`abstract_class` public abstract; extends `net/minecraft/client/gui/components/AbstractButton`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(IIIILnet/minecraft/network/chat/Component;Lnet/minecraft/client/gui/c` | exact | invokespecial@22 in `FabricCreativeGuiComponents$CreativeModeTabButton.<init>` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |
| calls | `builder` | `(Lnet/minecraft/network/chat/Component;Lnet/minecraft/client/gui/compo` | exact | invokestatic@71 in `EnumRuleEntry.<init>` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `builder` | `(Lnet/minecraft/network/chat/Component;Lnet/minecraft/client/gui/compo` | exact | invokestatic@36 in `DetailedBackupConfirmScreen.init` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `builder` | `(Lnet/minecraft/network/chat/Component;Lnet/minecraft/client/gui/compo` | exact | invokestatic@13 in `DetailsScreen.addFooter` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `extractRenderState` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V` | inherited_exact | invokevirtual@47 in `EnumRuleEntry.extractContent` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `getMessage` | `()Lnet/minecraft/network/chat/Component;` | inherited_exact | invokevirtual@24 in `ClientGameTestContextImpl.pressMatchingButton` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `onPress` | `(Lnet/minecraft/client/input/InputWithModifiers;)V` | exact | invokevirtual@40 in `ClientGameTestContextImpl.pressMatchingButton` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `setMessage` | `(Lnet/minecraft/network/chat/Component;)V` | inherited_exact | invokevirtual@51 in `EnumRuleEntry.lambda$new$0` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `setX` | `(I)V` | inherited_exact | invokevirtual@24 in `EnumRuleEntry.extractContent` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `setY` | `(I)V` | inherited_exact | invokevirtual@35 in `EnumRuleEntry.extractContent` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| reads | `DEFAULT_NARRATION` | `Lnet/minecraft/client/gui/components/Button$CreateNarration;` | exact | getstatic@19 in `FabricCreativeGuiComponents$CreativeModeTabButton.<init>` | unknown | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |

## Declared members (8 fields, 8 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final SMALL_WIDTH : I
public static final DEFAULT_WIDTH : I
public static final BIG_WIDTH : I
public static final DEFAULT_HEIGHT : I
public static final DEFAULT_SPACING : I
protected static final DEFAULT_NARRATION : Lnet/minecraft/client/gui/components/Button$CreateNarration;
protected final onPress : Lnet/minecraft/client/gui/components/Button$OnPress;
protected final createNarration : Lnet/minecraft/client/gui/components/Button$CreateNarration;
public static builder(Lnet/minecraft/network/chat/Component;Lnet/minecraft/client/gui/components/Button$OnPress;)Lnet/minecraft/client/gui/components/Button$Builder;
protected <init>(IIIILnet/minecraft/network/chat/Component;Lnet/minecraft/client/gui/components/Button$OnPress;Lnet/minecraft/client/gui/components/Button$CreateNarration;)V
public onPress(Lnet/minecraft/client/input/InputWithModifiers;)V
protected createNarrationMessage()Lnet/minecraft/network/chat/MutableComponent;
public updateWidgetNarration(Lnet/minecraft/client/gui/narration/NarrationElementOutput;)V
private synthetic lambda$createNarrationMessage$0()Lnet/minecraft/network/chat/MutableComponent;
private static synthetic lambda$static$0(Ljava/util/function/Supplier;)Lnet/minecraft/network/chat/MutableComponent;
static <clinit>()V
```
