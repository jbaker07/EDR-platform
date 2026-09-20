---
type: "interface"
fqcn: "net.minecraft.client.resources.sounds.SoundInstance"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.resources.sounds.SoundInstance

System: [[20-Systems/net.minecraft.client.resources|net.minecraft.client.resources]]

`interface` public abstract; extends `java/lang/Object`; implements `net/fabricmc/fabric/api/client/sound/v1/FabricSoundInstance`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getAudioStream` | `(Lnet/minecraft/client/sounds/SoundBufferLibrary;Lnet/minecraft/resour` | inherited_exact | invokeinterface@5 in `SoundEngineMixin.getStream` | unknown | [[30-Mechanisms/fabric-sound-api-v1|fabric-sound-api-v1]] | direct_reference |

## Declared members (0 fields, 17 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public abstract getIdentifier()Lnet/minecraft/resources/Identifier;
public abstract getOrResolve(Lnet/minecraft/client/sounds/SoundManager;)Lnet/minecraft/client/sounds/WeighedSoundEvents;
public abstract getSound()Lnet/minecraft/client/resources/sounds/Sound;
public abstract getSoundEvent()Lnet/minecraft/client/sounds/WeighedSoundEvents;
public abstract getSource()Lnet/minecraft/sounds/SoundSource;
public abstract isLooping()Z
public abstract isRelative()Z
public abstract getDelay()I
public abstract getVolume()F
public abstract getPitch()F
public abstract getX()D
public abstract getY()D
public abstract getZ()D
public abstract getAttenuation()Lnet/minecraft/client/resources/sounds/SoundInstance$Attenuation;
public canStartSilent()Z
public canPlaySound()Z
public static createUnseededRandom()Lnet/minecraft/util/RandomSource;
```
