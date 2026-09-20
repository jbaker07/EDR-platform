---
type: "interface"
fqcn: "net.fabricmc.fabric.api.client.screen.v1.ScreenEvents"
module: "fabric-screen-api-v1"
sha256: "6d0660544189cee8ed9a0d7a668ff69c30068799de633e4813924c0b6d3d49ed"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.screen.v1.ScreenEvents

Module: [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] -- kind: class

```java
public static final net.fabricmc.fabric.api.event.Event<net.fabricmc.fabric.api.client.screen.v1.ScreenEvents$BeforeInit> BEFORE_INIT
public static final net.fabricmc.fabric.api.event.Event<net.fabricmc.fabric.api.client.screen.v1.ScreenEvents$AfterInit> AFTER_INIT
public static net.fabricmc.fabric.api.event.Event<net.fabricmc.fabric.api.client.screen.v1.ScreenEvents$Remove> remove(net.minecraft.client.gui.screens.Screen)
public static net.fabricmc.fabric.api.event.Event<net.fabricmc.fabric.api.client.screen.v1.ScreenEvents$BeforeExtract> beforeExtract(net.minecraft.client.gui.screens.Screen)
public static net.fabricmc.fabric.api.event.Event<net.fabricmc.fabric.api.client.screen.v1.ScreenEvents$AfterBackground> afterBackground(net.minecraft.client.gui.screens.Screen)
public static net.fabricmc.fabric.api.event.Event<net.fabricmc.fabric.api.client.screen.v1.ScreenEvents$AfterForeground> afterForeground(net.minecraft.client.gui.screens.Screen)
public static net.fabricmc.fabric.api.event.Event<net.fabricmc.fabric.api.client.screen.v1.ScreenEvents$AfterExtract> afterExtract(net.minecraft.client.gui.screens.Screen)
public static net.fabricmc.fabric.api.event.Event<net.fabricmc.fabric.api.client.screen.v1.ScreenEvents$BeforeTick> beforeTick(net.minecraft.client.gui.screens.Screen)
public static net.fabricmc.fabric.api.event.Event<net.fabricmc.fabric.api.client.screen.v1.ScreenEvents$AfterTick> afterTick(net.minecraft.client.gui.screens.Screen)
static {}
```
