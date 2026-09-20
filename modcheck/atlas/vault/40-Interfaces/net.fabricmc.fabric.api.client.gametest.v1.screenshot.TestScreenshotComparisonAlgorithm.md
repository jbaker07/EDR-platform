---
type: "interface"
fqcn: "net.fabricmc.fabric.api.client.gametest.v1.screenshot.TestScreenshotComparisonAlgorithm"
module: "fabric-client-gametest-api-v1"
sha256: "09d7d48475eef47a347c12c51bd32c6cffc065a6785f3023f5aa6be4b815cd38"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.gametest.v1.screenshot.TestScreenshotComparisonAlgorithm

Module: [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] -- kind: interface

```java
public static net.fabricmc.fabric.api.client.gametest.v1.screenshot.TestScreenshotComparisonAlgorithm defaultAlgorithm()
public static net.fabricmc.fabric.api.client.gametest.v1.screenshot.TestScreenshotComparisonAlgorithm meanSquaredDifference(float)
public static net.fabricmc.fabric.api.client.gametest.v1.screenshot.TestScreenshotComparisonAlgorithm exact()
public abstract org.joml.Vector2i findColor(net.fabricmc.fabric.api.client.gametest.v1.screenshot.TestScreenshotComparisonAlgorithm$RawImage, net.fabricmc.fabric.api.client.gametest.v1.screenshot.TestScreenshotComparisonAlgorithm$RawImage)
public org.joml.Vector2i findGrayscale(net.fabricmc.fabric.api.client.gametest.v1.screenshot.TestScreenshotComparisonAlgorithm$RawImage, net.fabricmc.fabric.api.client.gametest.v1.screenshot.TestScreenshotComparisonAlgorithm$RawImage)
```
