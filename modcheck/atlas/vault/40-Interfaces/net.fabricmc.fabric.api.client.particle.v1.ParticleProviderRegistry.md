---
type: "interface"
fqcn: "net.fabricmc.fabric.api.client.particle.v1.ParticleProviderRegistry"
module: "fabric-particles-v1"
sha256: "0bf0c29bd7f1803eac7c5d4aec1af51bb25790ba41583108a2320740043cd0c2"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.particle.v1.ParticleProviderRegistry

Module: [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] -- kind: interface

```java
public static net.fabricmc.fabric.api.client.particle.v1.ParticleProviderRegistry getInstance()
public abstract <T extends net.minecraft.core.particles.ParticleOptions> void register(net.minecraft.core.particles.ParticleType<T>, net.minecraft.client.particle.ParticleProvider<T>)
public abstract <T extends net.minecraft.core.particles.ParticleOptions> void register(net.minecraft.core.particles.ParticleType<T>, net.fabricmc.fabric.api.client.particle.v1.ParticleProviderRegistry$PendingParticleProvider<T>)
```
