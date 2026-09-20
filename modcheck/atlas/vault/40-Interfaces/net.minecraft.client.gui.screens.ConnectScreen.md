---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.ConnectScreen"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.ConnectScreen

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `startConnecting(Lnet/minecraft/client/gui/screens/Screen;Lnet/minecraft/cli` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (22, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.gui.screens.ConnectScreen extends net.minecraft.client.gui.screens.Screen {
    private static final java.util.concurrent.atomic.AtomicInteger UNIQUE_THREAD_ID;
    private static final org.slf4j.Logger LOGGER;
    private static final long NARRATION_DELAY_MS;
    public static final net.minecraft.network.chat.Component ABORT_CONNECTION;
    public static final net.minecraft.network.chat.Component UNKNOWN_HOST_MESSAGE;
    private volatile net.minecraft.network.Connection connection;
    private io.netty.channel.ChannelFuture channelFuture;
    private volatile boolean aborted;
    private final net.minecraft.client.gui.screens.Screen parent;
    private net.minecraft.network.chat.Component status;
    private long lastNarration;
    private final net.minecraft.network.chat.Component connectFailedTitle;
    private net.minecraft.client.gui.screens.ConnectScreen(net.minecraft.client.gui.screens.Screen, net.minecraft.network.chat.Component);
    public static void startConnecting(net.minecraft.client.gui.screens.Screen, net.minecraft.client.Minecraft, net.minecraft.client.multiplayer.resolver.ServerAddress, net.minecraft.client.multiplayer.ServerData, boolean, net.minecraft.client.multiplayer.TransferState);
    private void connect(net.minecraft.client.Minecraft, net.minecraft.client.multiplayer.resolver.ServerAddress, net.minecraft.client.multiplayer.ServerData, net.minecraft.client.multiplayer.TransferState);
    private void updateStatus(net.minecraft.network.chat.Component);
    public void tick();
    public boolean shouldCloseOnEsc();
    protected void init();
    public void extractRenderState(net.minecraft.client.gui.GuiGraphicsExtractor, int, int, float);
    private void lambda$init$0(net.minecraft.client.gui.components.Button);
    static {};
}
```
