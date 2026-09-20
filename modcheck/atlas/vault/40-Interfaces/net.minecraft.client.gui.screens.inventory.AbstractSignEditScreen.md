---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.inventory.AbstractSignEditScreen"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.inventory.AbstractSignEditScreen

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/world/level/block/entity/SignBlockEntity;Lne` | `` | client | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |
| calls | `"<init>"(Lnet/minecraft/world/level/block/entity/SignBlockEntity;Lne` | `` | client | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |

## Declared members (36, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public abstract class net.minecraft.client.gui.screens.inventory.AbstractSignEditScreen extends net.minecraft.client.gui.screens.Screen {
    private static final int LINE_COUNT;
    protected final net.minecraft.world.level.block.entity.SignBlockEntity sign;
    private final net.minecraft.world.level.block.entity.SignText$Mutable text;
    private final java.lang.String[] messages;
    private final net.minecraft.world.level.block.entity.SignTextSlot slot;
    private final int textColor;
    protected final net.minecraft.world.level.block.state.properties.WoodType woodType;
    private long cursorBlinkStartTime;
    private int line;
    private final net.minecraft.client.gui.font.TextFieldHelper signField;
    private net.minecraft.client.gui.components.IMEPreeditOverlay preeditOverlay;
    private final org.joml.Vector2f cursorPosScratch;
    public net.minecraft.client.gui.screens.inventory.AbstractSignEditScreen(net.minecraft.world.level.block.entity.SignBlockEntity, net.minecraft.world.level.block.entity.SignTextSlot, boolean);
    public net.minecraft.client.gui.screens.inventory.AbstractSignEditScreen(net.minecraft.world.level.block.entity.SignBlockEntity, net.minecraft.world.level.block.entity.SignTextSlot, boolean, net.minecraft.network.chat.Component);
    protected void init();
    public void tick();
    private boolean isValid();
    public boolean keyPressed(net.minecraft.client.input.KeyEvent);
    public boolean isInputCaptured();
    public boolean charTyped(net.minecraft.client.input.CharacterEvent);
    public boolean preeditUpdated(net.minecraft.client.input.PreeditEvent);
    public void extractRenderState(net.minecraft.client.gui.GuiGraphicsExtractor, int, int, float);
    public void onClose();
    public void removed();
    public boolean isPauseScreen();
    public boolean isInGameUi();
    protected abstract void extractSignBackground(net.minecraft.client.gui.GuiGraphicsExtractor);
    protected abstract org.joml.Vector3fc getSignTextScale();
    protected abstract float getSignYOffset();
    private void extractSign(net.minecraft.client.gui.GuiGraphicsExtractor);
    private void extractSignText(net.minecraft.client.gui.GuiGraphicsExtractor, org.joml.Vector2f);
    private void setMessage(java.lang.String);
    private void onDone();
    private void lambda$init$0(net.minecraft.client.gui.components.Button);
    private boolean lambda$new$1(net.minecraft.world.level.block.entity.SignBlockEntity, java.lang.String);
    private java.lang.String lambda$new$0();
}
```
