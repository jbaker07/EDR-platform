---
type: "interface"
fqcn: "net.fabricmc.fabric.api.resource.v1.ResourceLoader"
module: "fabric-resource-loader-v1"
sha256: "2d2fb907728c895640c1261ff3dd115b5087913d88d1ba1d9f3a8a4cfd1ca87e"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.resource.v1.ResourceLoader

Module: [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] -- kind: interface

```java
public static final net.minecraft.server.packs.resources.PreparableReloadListener$StateKey<net.minecraft.core.HolderLookup$Provider> REGISTRY_LOOKUP_KEY
public static final net.minecraft.server.packs.resources.PreparableReloadListener$StateKey<net.minecraft.world.flag.FeatureFlagSet> FEATURE_FLAG_SET_KEY
public static net.fabricmc.fabric.api.resource.v1.ResourceLoader get(net.minecraft.server.packs.PackType)
public abstract void registerReloadListener(net.minecraft.resources.Identifier, net.minecraft.server.packs.resources.PreparableReloadListener)
public abstract void addListenerOrdering(net.minecraft.resources.Identifier, net.minecraft.resources.Identifier)
public static boolean registerBuiltinPack(net.minecraft.resources.Identifier, net.fabricmc.loader.api.ModContainer, net.fabricmc.fabric.api.resource.v1.pack.PackActivationType)
public static boolean registerBuiltinPack(net.minecraft.resources.Identifier, net.fabricmc.loader.api.ModContainer, net.minecraft.network.chat.Component, net.fabricmc.fabric.api.resource.v1.pack.PackActivationType)
static {}
```
