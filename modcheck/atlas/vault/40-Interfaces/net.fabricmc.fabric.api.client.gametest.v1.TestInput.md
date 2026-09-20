---
type: "interface"
fqcn: "net.fabricmc.fabric.api.client.gametest.v1.TestInput"
module: "fabric-client-gametest-api-v1"
sha256: "09d7d48475eef47a347c12c51bd32c6cffc065a6785f3023f5aa6be4b815cd38"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.gametest.v1.TestInput

Module: [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] -- kind: interface

```java
public abstract void holdKey(net.minecraft.client.KeyMapping)
public abstract void holdKey(java.util.function.Function)
public abstract void holdKey(com.mojang.blaze3d.platform.InputConstants$Key)
public abstract void holdKey(int)
public abstract void holdMouse(int)
public abstract void holdControl()
public abstract void holdShift()
public abstract void holdAlt()
public abstract void releaseKey(net.minecraft.client.KeyMapping)
public abstract void releaseKey(java.util.function.Function)
public abstract void releaseKey(com.mojang.blaze3d.platform.InputConstants$Key)
public abstract void releaseKey(int)
public abstract void releaseMouse(int)
public abstract void releaseControl()
public abstract void releaseShift()
public abstract void releaseAlt()
public abstract void pressKey(net.minecraft.client.KeyMapping)
public abstract void pressKey(java.util.function.Function)
public abstract void pressKey(com.mojang.blaze3d.platform.InputConstants$Key)
public abstract void pressKey(int)
public abstract void pressMouse(int)
public abstract void holdKeyFor(net.minecraft.client.KeyMapping, int)
public abstract void holdKeyFor(java.util.function.Function, int)
public abstract void holdKeyFor(com.mojang.blaze3d.platform.InputConstants$Key, int)
public abstract void holdKeyFor(int, int)
public abstract void holdMouseFor(int, int)
public abstract void lookAt(float, float)
public abstract void lookAt(net.minecraft.core.BlockPos)
public abstract void typeChar(int)
public abstract void typeChars(java.lang.String)
public abstract void scroll(double)
public abstract void scroll(double, double)
public abstract void setCursorPos(double, double)
public abstract void moveCursor(double, double)
public abstract void resizeWindow(int, int)
```
