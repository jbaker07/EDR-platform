---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.BackupConfirmScreen"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.BackupConfirmScreen

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Ljava/lang/Runnable;Lnet/minecraft/client/gui/screens/Backu` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `init()V` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (20, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.gui.screens.BackupConfirmScreen extends net.minecraft.client.gui.screens.Screen {
    private static final net.minecraft.network.chat.Component SKIP_AND_JOIN;
    public static final net.minecraft.network.chat.Component BACKUP_AND_JOIN;
    private final java.lang.Runnable onCancel;
    protected final net.minecraft.client.gui.screens.BackupConfirmScreen$Listener onProceed;
    private final net.minecraft.network.chat.Component description;
    private final boolean promptForCacheErase;
    private net.minecraft.client.gui.components.MultiLineLabel message;
    private final net.minecraft.network.chat.Component confirmation;
    protected int id;
    private net.minecraft.client.gui.components.Checkbox eraseCache;
    public net.minecraft.client.gui.screens.BackupConfirmScreen(java.lang.Runnable, net.minecraft.client.gui.screens.BackupConfirmScreen$Listener, net.minecraft.network.chat.Component, net.minecraft.network.chat.Component, boolean);
    public net.minecraft.client.gui.screens.BackupConfirmScreen(java.lang.Runnable, net.minecraft.client.gui.screens.BackupConfirmScreen$Listener, net.minecraft.network.chat.Component, net.minecraft.network.chat.Component, net.minecraft.network.chat.Component, boolean);
    protected void init();
    public void extractRenderState(net.minecraft.client.gui.GuiGraphicsExtractor, int, int, float);
    public boolean shouldCloseOnEsc();
    public boolean keyPressed(net.minecraft.client.input.KeyEvent);
    private void lambda$init$2(net.minecraft.client.gui.components.Button);
    private void lambda$init$1(net.minecraft.client.gui.components.Button);
    private void lambda$init$0(net.minecraft.client.gui.components.Button);
    static {};
}
```
