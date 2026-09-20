---
type: "interface"
fqcn: "net.fabricmc.fabric.api.resource.ResourceManagerHelper"
module: "fabric-resource-loader-v0"
sha256: "18afa6466d69ff68e1eeb74088d210a55004d11380ec14aad6ea66faa7da1922"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.resource.ResourceManagerHelper

Module: [[30-Mechanisms/fabric-resource-loader-v0|fabric-resource-loader-v0]] -- kind: interface

```java
public void addReloadListener(net.fabricmc.fabric.api.resource.IdentifiableResourceReloadListener)
public abstract void registerReloadListener(net.fabricmc.fabric.api.resource.IdentifiableResourceReloadListener)
public abstract void registerReloadListener(net.minecraft.resources.Identifier, java.util.function.Function)
public static net.fabricmc.fabric.api.resource.ResourceManagerHelper get(net.minecraft.server.packs.PackType)
public static boolean registerBuiltinResourcePack(net.minecraft.resources.Identifier, net.fabricmc.loader.api.ModContainer, net.fabricmc.fabric.api.resource.ResourcePackActivationType)
public static boolean registerBuiltinResourcePack(net.minecraft.resources.Identifier, net.fabricmc.loader.api.ModContainer, net.minecraft.network.chat.Component, net.fabricmc.fabric.api.resource.ResourcePackActivationType)
public static boolean registerBuiltinResourcePack(net.minecraft.resources.Identifier, net.fabricmc.loader.api.ModContainer, java.lang.String, net.fabricmc.fabric.api.resource.ResourcePackActivationType)
public static boolean registerBuiltinResourcePack(net.minecraft.resources.Identifier, java.lang.String, net.fabricmc.loader.api.ModContainer, boolean)
```
