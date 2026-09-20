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
public abstract int waitFor(java.util.function.Predicate)
public abstract int waitFor(java.util.function.Predicate, int)
public abstract int waitForScreen(java.lang.Class)
public abstract void setScreen(java.util.function.Supplier)
public abstract void clickScreenButton(java.lang.String)
public abstract boolean tryClickScreenButton(java.lang.String)
public java.nio.file.Path takeScreenshot(java.lang.String)
public abstract java.nio.file.Path takeScreenshot(net.fabricmc.fabric.api.client.gametest.v1.screenshot.TestScreenshotOptions)
public void assertScreenshotEquals(java.lang.String)
public abstract void assertScreenshotEquals(net.fabricmc.fabric.api.client.gametest.v1.screenshot.TestScreenshotComparisonOptions)
public org.joml.Vector2i assertScreenshotContains(java.lang.String)
public abstract org.joml.Vector2i assertScreenshotContains(net.fabricmc.fabric.api.client.gametest.v1.screenshot.TestScreenshotComparisonOptions)
public abstract net.fabricmc.fabric.api.client.gametest.v1.TestInput getInput()
public abstract net.fabricmc.fabric.api.client.gametest.v1.world.TestWorldBuilder worldBuilder()
public abstract void restoreDefaultGameOptions()
public abstract void runOnClient(org.apache.commons.lang3.function.FailableConsumer)
public abstract java.lang.Object computeOnClient(org.apache.commons.lang3.function.FailableFunction)
```
