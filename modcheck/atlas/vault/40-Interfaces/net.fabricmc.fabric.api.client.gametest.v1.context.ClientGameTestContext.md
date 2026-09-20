---
type: "interface"
fqcn: "net.fabricmc.fabric.api.client.gametest.v1.context.ClientGameTestContext"
module: "fabric-client-gametest-api-v1"
sha256: "09d7d48475eef47a347c12c51bd32c6cffc065a6785f3023f5aa6be4b815cd38"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.gametest.v1.context.ClientGameTestContext

Module: [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] -- kind: interface

```java
public static final int NO_TIMEOUT
public static final int DEFAULT_TIMEOUT
public abstract void waitTick()
public abstract void waitTicks(int)
public abstract int waitFor(java.util.function.Predicate<net.minecraft.client.Minecraft>)
public abstract int waitFor(java.util.function.Predicate<net.minecraft.client.Minecraft>, int)
public abstract int waitForScreen(java.lang.Class<? extends net.minecraft.client.gui.screens.Screen>)
public abstract void setScreen(java.util.function.Supplier<net.minecraft.client.gui.screens.Screen>)
public abstract void clickScreenButton(java.lang.String)
public abstract boolean tryClickScreenButton(java.lang.String)
public default java.nio.file.Path takeScreenshot(java.lang.String)
public abstract java.nio.file.Path takeScreenshot(net.fabricmc.fabric.api.client.gametest.v1.screenshot.TestScreenshotOptions)
public default void assertScreenshotEquals(java.lang.String)
public abstract void assertScreenshotEquals(net.fabricmc.fabric.api.client.gametest.v1.screenshot.TestScreenshotComparisonOptions)
public default org.joml.Vector2i assertScreenshotContains(java.lang.String)
public abstract org.joml.Vector2i assertScreenshotContains(net.fabricmc.fabric.api.client.gametest.v1.screenshot.TestScreenshotComparisonOptions)
public abstract net.fabricmc.fabric.api.client.gametest.v1.TestInput getInput()
public abstract net.fabricmc.fabric.api.client.gametest.v1.world.TestWorldBuilder worldBuilder()
public abstract void restoreDefaultGameOptions()
public abstract <E extends java.lang.Throwable> void runOnClient(org.apache.commons.lang3.function.FailableConsumer<net.minecraft.client.Minecraft, E>) throws E
public abstract <T, E extends java.lang.Throwable> T computeOnClient(org.apache.commons.lang3.function.FailableFunction<net.minecraft.client.Minecraft, T, E>) throws E
```
