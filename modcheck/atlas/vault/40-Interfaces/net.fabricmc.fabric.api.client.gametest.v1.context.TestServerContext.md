---
type: "interface"
fqcn: "net.fabricmc.fabric.api.client.gametest.v1.context.TestServerContext"
module: "fabric-client-gametest-api-v1"
sha256: "09d7d48475eef47a347c12c51bd32c6cffc065a6785f3023f5aa6be4b815cd38"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.gametest.v1.context.TestServerContext

Module: [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] -- kind: interface

```java
public abstract void runCommand(java.lang.String)
public abstract <E extends java.lang.Throwable> void runOnServer(org.apache.commons.lang3.function.FailableConsumer<net.minecraft.server.MinecraftServer, E>) throws E
public abstract <T, E extends java.lang.Throwable> T computeOnServer(org.apache.commons.lang3.function.FailableFunction<net.minecraft.server.MinecraftServer, T, E>) throws E
public abstract int waitFor(java.util.function.Predicate<net.minecraft.server.MinecraftServer>)
public abstract int waitFor(java.util.function.Predicate<net.minecraft.server.MinecraftServer>, int)
```
