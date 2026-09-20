---
type: "interface"
fqcn: "net.minecraft.client.gui.components.Button"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.components.Button

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `builder(Lnet/minecraft/network/chat/Component;Lnet/minecraft/client` | `` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `builder(Lnet/minecraft/network/chat/Component;Lnet/minecraft/client` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `builder(Lnet/minecraft/network/chat/Component;Lnet/minecraft/client` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `extractRenderState(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V` | `` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `getMessage()Lnet/minecraft/network/chat/Component;` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `onPress(Lnet/minecraft/client/input/InputWithModifiers;)V` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `setMessage(Lnet/minecraft/network/chat/Component;)V` | `` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `setX(I)V` | `` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |
| calls | `setY(I)V` | `` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |

## Declared members (16, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public abstract class net.minecraft.client.gui.components.Button extends net.minecraft.client.gui.components.AbstractButton {
    public static final int SMALL_WIDTH;
    public static final int DEFAULT_WIDTH;
    public static final int BIG_WIDTH;
    public static final int DEFAULT_HEIGHT;
    public static final int DEFAULT_SPACING;
    protected static final net.minecraft.client.gui.components.Button$CreateNarration DEFAULT_NARRATION;
    protected final net.minecraft.client.gui.components.Button$OnPress onPress;
    protected final net.minecraft.client.gui.components.Button$CreateNarration createNarration;
    public static net.minecraft.client.gui.components.Button$Builder builder(net.minecraft.network.chat.Component, net.minecraft.client.gui.components.Button$OnPress);
    protected net.minecraft.client.gui.components.Button(int, int, int, int, net.minecraft.network.chat.Component, net.minecraft.client.gui.components.Button$OnPress, net.minecraft.client.gui.components.Button$CreateNarration);
    public void onPress(net.minecraft.client.input.InputWithModifiers);
    protected net.minecraft.network.chat.MutableComponent createNarrationMessage();
    public void updateWidgetNarration(net.minecraft.client.gui.narration.NarrationElementOutput);
    private net.minecraft.network.chat.MutableComponent lambda$createNarrationMessage$0();
    private static net.minecraft.network.chat.MutableComponent lambda$static$0(java.util.function.Supplier);
    static {};
}
```
