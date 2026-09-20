---
type: "interface"
fqcn: "net.minecraft.client.resources.sounds.SoundInstance"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.resources.sounds.SoundInstance

System: [[20-Systems/net.minecraft.client.resources|net.minecraft.client.resources]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getAudioStream(Lnet/minecraft/client/sounds/SoundBufferLibrary;Lnet/minecr` | `` | client | [[30-Mechanisms/fabric-sound-api-v1|fabric-sound-api-v1]] | direct_reference |

## Declared members (17, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.client.resources.sounds.SoundInstance {
    public abstract net.minecraft.resources.Identifier getIdentifier();
    public abstract net.minecraft.client.sounds.WeighedSoundEvents getOrResolve(net.minecraft.client.sounds.SoundManager);
    public abstract net.minecraft.client.resources.sounds.Sound getSound();
    public abstract net.minecraft.client.sounds.WeighedSoundEvents getSoundEvent();
    public abstract net.minecraft.sounds.SoundSource getSource();
    public abstract boolean isLooping();
    public abstract boolean isRelative();
    public abstract int getDelay();
    public abstract float getVolume();
    public abstract float getPitch();
    public abstract double getX();
    public abstract double getY();
    public abstract double getZ();
    public abstract net.minecraft.client.resources.sounds.SoundInstance$Attenuation getAttenuation();
    public default boolean canStartSilent();
    public default boolean canPlaySound();
    public static net.minecraft.util.RandomSource createUnseededRandom();
}
```
