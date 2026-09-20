---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.inventory.AbstractRecipeBookScreen"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.inventory.AbstractRecipeBookScreen

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `extractRenderState` | `@Inject at INVOKE Lnet/minecraft/client/gui/screens/recipebook/RecipeBookCompone` | both | [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] | direct_reference |

## Declared members (22, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public abstract class net.minecraft.client.gui.screens.inventory.AbstractRecipeBookScreen<T extends net.minecraft.world.inventory.RecipeBookMenu> extends net.minecraft.client.gui.screens.inventory.AbstractContainerScreen<T> implements net.minecraft.client.gui.screens.recipebook.RecipeUpdateListener {
    private final net.minecraft.client.gui.screens.recipebook.RecipeBookComponent<?> recipeBookComponent;
    private boolean widthTooNarrow;
    public net.minecraft.client.gui.screens.inventory.AbstractRecipeBookScreen(T, net.minecraft.client.gui.screens.recipebook.RecipeBookComponent<?>, net.minecraft.world.entity.player.Inventory, net.minecraft.network.chat.Component);
    protected void init();
    protected abstract net.minecraft.client.gui.navigation.ScreenPosition getRecipeBookButtonPosition();
    private void initButton();
    protected void onRecipeBookButtonClick();
    public void extractRenderState(net.minecraft.client.gui.GuiGraphicsExtractor, int, int, float);
    protected void extractSlots(net.minecraft.client.gui.GuiGraphicsExtractor, int, int);
    protected boolean isBiggerResultSlot();
    public boolean charTyped(net.minecraft.client.input.CharacterEvent);
    public boolean keyPressed(net.minecraft.client.input.KeyEvent);
    public boolean isInputCaptured();
    public boolean mouseClicked(net.minecraft.client.input.MouseButtonEvent, boolean);
    public boolean mouseDragged(net.minecraft.client.input.MouseButtonEvent, double, double);
    protected boolean isHovering(int, int, int, int, double, double);
    protected boolean hasClickedOutside(double, double, int, int);
    protected void slotClicked(net.minecraft.world.inventory.Slot, int, int, net.minecraft.world.inventory.ContainerInput);
    public void containerTick();
    public void recipesUpdated();
    public void fillGhostRecipe(net.minecraft.world.item.crafting.display.RecipeDisplay);
    private void lambda$initButton$0(net.minecraft.client.gui.components.Button);
}
```
