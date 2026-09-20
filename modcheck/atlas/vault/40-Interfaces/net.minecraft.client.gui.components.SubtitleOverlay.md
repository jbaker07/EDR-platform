---
type: "interface"
fqcn: "net.minecraft.client.gui.components.SubtitleOverlay"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.components.SubtitleOverlay

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/client/sounds/SoundEventListener`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| wraps | `extractRenderState` | `(Lnet/minecraft/client/gui/GuiGraphicsExtractor;)V` | name_only | @WrapMethod | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (6 fields, 3 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOOPING_DISPLAY_FULL_BRIGHTNESS_TIME : I
private static final DISPLAY_TIME : J
private final minecraft : Lnet/minecraft/client/Minecraft;
private final subtitles : Ljava/util/List;
private isListening : Z
private final audibleSubtitles : Ljava/util/List;
public <init>(Lnet/minecraft/client/Minecraft;)V
public extractRenderState(Lnet/minecraft/client/gui/GuiGraphicsExtractor;)V
public onPlaySound(Lnet/minecraft/client/resources/sounds/SoundInstance;Lnet/minecraft/client/sounds/WeighedSoundEvents;F)V
```
