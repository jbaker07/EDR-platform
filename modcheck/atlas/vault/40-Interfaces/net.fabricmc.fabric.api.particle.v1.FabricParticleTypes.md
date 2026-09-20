---
type: "interface"
fqcn: "net.fabricmc.fabric.api.particle.v1.FabricParticleTypes"
module: "fabric-particles-v1"
sha256: "0bf0c29bd7f1803eac7c5d4aec1af51bb25790ba41583108a2320740043cd0c2"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.particle.v1.FabricParticleTypes

Module: [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] -- kind: class

```java
public static net.minecraft.core.particles.SimpleParticleType simple()
public static net.minecraft.core.particles.SimpleParticleType simple(boolean)
public static <T extends net.minecraft.core.particles.ParticleOptions> net.minecraft.core.particles.ParticleType<T> complex(com.mojang.serialization.MapCodec<T>, net.minecraft.network.codec.StreamCodec<? super net.minecraft.network.RegistryFriendlyByteBuf, T>)
public static <T extends net.minecraft.core.particles.ParticleOptions> net.minecraft.core.particles.ParticleType<T> complex(boolean, com.mojang.serialization.MapCodec<T>, net.minecraft.network.codec.StreamCodec<? super net.minecraft.network.RegistryFriendlyByteBuf, T>)
public static <T extends net.minecraft.core.particles.ParticleOptions> net.minecraft.core.particles.ParticleType<T> complex(java.util.function.Function<net.minecraft.core.particles.ParticleType<T>, com.mojang.serialization.MapCodec<T>>, java.util.function.Function<net.minecraft.core.particles.ParticleType<T>, net.minecraft.network.codec.StreamCodec<? super net.minecraft.network.RegistryFriendlyByteBuf, T>>)
public static <T extends net.minecraft.core.particles.ParticleOptions> net.minecraft.core.particles.ParticleType<T> complex(boolean, java.util.function.Function<net.minecraft.core.particles.ParticleType<T>, com.mojang.serialization.MapCodec<T>>, java.util.function.Function<net.minecraft.core.particles.ParticleType<T>, net.minecraft.network.codec.StreamCodec<? super net.minecraft.network.RegistryFriendlyByteBuf, T>>)
```
