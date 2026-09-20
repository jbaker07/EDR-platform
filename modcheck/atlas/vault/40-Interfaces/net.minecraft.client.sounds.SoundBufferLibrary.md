---
type: "interface"
fqcn: "net.minecraft.client.sounds.SoundBufferLibrary"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.sounds.SoundBufferLibrary

System: [[20-Systems/net.minecraft.client.sounds|net.minecraft.client.sounds]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getStream` | `(Lnet/minecraft/resources/Identifier;Z)Ljava/util/concurrent/Completab` | exact | invokevirtual@3 in `FabricSoundInstance.getAudioStream` | unknown | [[30-Mechanisms/fabric-sound-api-v1|fabric-sound-api-v1]] | direct_reference |

## Declared members (2 fields, 13 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final resourceManager : Lnet/minecraft/server/packs/resources/ResourceProvider;
private final cache : Ljava/util/Map;
public <init>(Lnet/minecraft/server/packs/resources/ResourceProvider;)V
public getCompleteBuffer(Lnet/minecraft/resources/Identifier;)Ljava/util/concurrent/CompletableFuture;
public getStream(Lnet/minecraft/resources/Identifier;Z)Ljava/util/concurrent/CompletableFuture;
public clear()V
public preload(Ljava/util/Collection;)Ljava/util/concurrent/CompletableFuture;
public enumerate(Lnet/minecraft/client/sounds/SoundBufferLibrary$DebugOutput;)V
private static synthetic lambda$enumerate$0(Lnet/minecraft/client/sounds/SoundBufferLibrary$DebugOutput;Lnet/minecraft/resources/Identifier;Ljava/util/concurrent/CompletableFuture;)V
private static synthetic lambda$preload$1(I)[Ljava/util/concurrent/CompletableFuture;
private synthetic lambda$preload$0(Lnet/minecraft/client/resources/sounds/Sound;)Ljava/util/concurrent/CompletableFuture;
private static synthetic lambda$clear$0(Ljava/util/concurrent/CompletableFuture;)V
private synthetic lambda$getStream$0(Lnet/minecraft/resources/Identifier;Z)Lnet/minecraft/client/sounds/AudioStream;
private synthetic lambda$getCompleteBuffer$0(Lnet/minecraft/resources/Identifier;)Ljava/util/concurrent/CompletableFuture;
private synthetic lambda$getCompleteBuffer$1(Lnet/minecraft/resources/Identifier;)Lcom/mojang/blaze3d/audio/SoundBuffer;
```
