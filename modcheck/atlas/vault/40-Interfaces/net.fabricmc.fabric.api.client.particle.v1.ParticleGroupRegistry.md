---
type: "interface"
fqcn: "net.fabricmc.fabric.api.client.particle.v1.ParticleGroupRegistry"
module: "fabric-particles-v1"
sha256: "0bf0c29bd7f1803eac7c5d4aec1af51bb25790ba41583108a2320740043cd0c2"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.particle.v1.ParticleGroupRegistry

Module: [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] -- kind: class

```java
public static void register(net.minecraft.client.particle.ParticleRenderType, java.util.function.Function<net.minecraft.client.particle.ParticleEngine, net.minecraft.client.particle.ParticleGroup<?>>)
public static void registerOrdering(net.minecraft.client.particle.ParticleRenderType, net.minecraft.resources.Identifier)
public static void registerOrdering(net.minecraft.client.particle.ParticleRenderType, net.minecraft.client.particle.ParticleRenderType)
public static void registerOrdering(net.minecraft.resources.Identifier, net.minecraft.client.particle.ParticleRenderType)
public static void registerOrdering(net.minecraft.resources.Identifier, net.minecraft.resources.Identifier)
public static net.minecraft.client.particle.ParticleRenderType getParticleRenderType(net.minecraft.resources.Identifier)
public static net.minecraft.resources.Identifier getId(net.minecraft.client.particle.ParticleRenderType)
```
