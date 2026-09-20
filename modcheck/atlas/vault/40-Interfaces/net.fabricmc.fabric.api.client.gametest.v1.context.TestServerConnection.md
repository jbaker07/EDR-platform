---
type: "interface"
fqcn: "net.fabricmc.fabric.api.client.gametest.v1.context.TestServerConnection"
module: "fabric-client-gametest-api-v1"
sha256: "09d7d48475eef47a347c12c51bd32c6cffc065a6785f3023f5aa6be4b815cd38"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.gametest.v1.context.TestServerConnection

Module: [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] -- kind: interface

```java
public static final int DEFAULT_CHUNK_LOAD_TIMEOUT
public default int waitForChunksDownload()
public abstract int waitForChunksDownload(int)
public default int waitForChunksRender()
public default int waitForChunksRender(int)
public default int waitForChunksRender(boolean)
public abstract int waitForChunksRender(boolean, int)
public abstract void waitForClientboundPackets()
public abstract void waitForServerboundPackets()
public abstract void waitForClientboundEntityUpdates(net.minecraft.world.entity.EntityType<?>, net.minecraft.world.entity.EntityType<?>...)
public abstract net.minecraft.client.player.LocalPlayer getClientPlayer()
public abstract net.minecraft.server.level.ServerPlayer getServerPlayer()
public abstract net.minecraft.client.multiplayer.ClientLevel getClientLevel()
public abstract net.minecraft.server.level.ServerLevel getServerLevel()
```
