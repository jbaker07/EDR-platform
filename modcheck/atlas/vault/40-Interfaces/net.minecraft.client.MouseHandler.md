---
type: "interface"
fqcn: "net.minecraft.client.MouseHandler"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.MouseHandler

System: [[20-Systems/net.minecraft.client|net.minecraft.client]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `xpos()D` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `ypos()D` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (47, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.MouseHandler {
    private static final org.slf4j.Logger LOGGER;
    public static final long DOUBLE_CLICK_THRESHOLD_MS;
    private final net.minecraft.client.Minecraft minecraft;
    private boolean isLeftPressed;
    private boolean isMiddlePressed;
    private boolean isRightPressed;
    private double xpos;
    private double ypos;
    private net.minecraft.client.MouseHandler$LastClick lastClick;
    protected int lastClickButton;
    private net.minecraft.client.input.MouseButtonInfo activeButton;
    private boolean ignoreFirstMove;
    private double mousePressedTime;
    private final net.minecraft.util.SmoothDouble smoothTurnX;
    private final net.minecraft.util.SmoothDouble smoothTurnY;
    private double accumulatedDX;
    private double accumulatedDY;
    private final net.minecraft.client.ScrollWheelHandler scrollWheelHandler;
    private double lastHandleMovementTime;
    private boolean mouseGrabbed;
    public net.minecraft.client.MouseHandler(net.minecraft.client.Minecraft);
    public void onButton(long, net.minecraft.client.input.MouseButtonInfo, int);
    public void fillMousePositionDetails(net.minecraft.CrashReportCategory, com.mojang.blaze3d.platform.Window);
    public void onScroll(long, double, double);
    public void onDrop(long, java.util.List<java.lang.String>);
    public void onMove(long, double, double, double, double);
    public void handleAccumulatedMovement();
    public static double getScaledXPos(com.mojang.blaze3d.platform.Window, double);
    public double getScaledXPos(com.mojang.blaze3d.platform.Window);
    public static double getScaledYPos(com.mojang.blaze3d.platform.Window, double);
    public double getScaledYPos(com.mojang.blaze3d.platform.Window);
    private void turnPlayer(double);
    public boolean isLeftPressed();
    public boolean isMiddlePressed();
    public boolean isRightPressed();
    public double xpos();
    public double ypos();
    public void setIgnoreFirstMove();
    public void resyncMousePosition();
    public boolean isMouseGrabbed();
    public void grabMouse();
    public void releaseMouse();
    public void cursorEntered();
    public void drawDebugMouseInfo(net.minecraft.client.gui.Font, net.minecraft.client.gui.GuiGraphicsExtractor);
    private static java.lang.String lambda$fillMousePositionDetails$1(com.mojang.blaze3d.platform.Window) throws java.lang.Exception;
    private java.lang.String lambda$fillMousePositionDetails$0(com.mojang.blaze3d.platform.Window) throws java.lang.Exception;
    static {};
}
```
