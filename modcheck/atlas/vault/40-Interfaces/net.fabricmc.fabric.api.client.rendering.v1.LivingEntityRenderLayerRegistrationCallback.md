---
type: "interface"
fqcn: "net.fabricmc.fabric.api.client.rendering.v1.LivingEntityRenderLayerRegistrationCallback"
module: "fabric-rendering-v1"
sha256: "749427999b4845b129683b1db268a04b524abb6ab351dcaf67cda9a3ab56b5c0"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.rendering.v1.LivingEntityRenderLayerRegistrationCallback

Module: [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] -- kind: interface

```java
public static final net.fabricmc.fabric.api.event.Event<net.fabricmc.fabric.api.client.rendering.v1.LivingEntityRenderLayerRegistrationCallback> EVENT
public abstract void registerLayers(net.minecraft.world.entity.EntityType<? extends net.minecraft.world.entity.LivingEntity>, net.minecraft.client.renderer.entity.LivingEntityRenderer<?, ?, ?>, net.fabricmc.fabric.api.client.rendering.v1.LivingEntityRenderLayerRegistrationCallback$RegistrationHelper, net.minecraft.client.renderer.entity.EntityRendererProvider$Context)
static {}
```
