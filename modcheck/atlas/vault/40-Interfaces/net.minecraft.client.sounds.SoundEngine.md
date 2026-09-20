---
type: "interface"
fqcn: "net.minecraft.client.sounds.SoundEngine"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.sounds.SoundEngine

System: [[20-Systems/net.minecraft.client.sounds|net.minecraft.client.sounds]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| wraps | `play` | `(Lnet/minecraft/client/resources/sounds/SoundInstance;)Lnet/minecraft/` | exact | @Redirect at ['INVOKE'] | client | [[30-Mechanisms/fabric-sound-api-v1|fabric-sound-api-v1]] | direct_reference |

## Declared members (32 fields, 48 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final MARKER : Lorg/slf4j/Marker;
private static final LOGGER : Lorg/slf4j/Logger;
private static final PITCH_MIN : F
private static final PITCH_MAX : F
private static final VOLUME_MIN : F
private static final VOLUME_MAX : F
private static final MIN_SOURCE_LIFETIME : I
private static final ONLY_WARN_ONCE : Ljava/util/Set;
public static final MISSING_SOUND : Ljava/lang/String;
public static final LOOPING_SOUND_SUBTITLE_INTERVAL_TICKS : I
public static final OPEN_AL_SOFT_PREFIX : Ljava/lang/String;
public static final OPEN_AL_SOFT_PREFIX_LENGTH : I
private final soundManager : Lnet/minecraft/client/sounds/SoundManager;
private final options : Lnet/minecraft/client/Options;
private loaded : Z
private final library : Lcom/mojang/blaze3d/audio/Library;
private final listener : Lcom/mojang/blaze3d/audio/Listener;
private final soundBuffers : Lnet/minecraft/client/sounds/SoundBufferLibrary;
private final executor : Lnet/minecraft/client/sounds/SoundEngineExecutor;
private final channelAccess : Lnet/minecraft/client/sounds/ChannelAccess;
private tickCount : I
private lastSeenDevices : Lcom/mojang/blaze3d/audio/DeviceList;
private final deviceTracker : Lcom/mojang/blaze3d/audio/DeviceTracker;
private final instanceToChannel : Ljava/util/Map;
private final instanceBySource : Lcom/google/common/collect/Multimap;
private final gainBySource : Lit/unimi/dsi/fastutil/objects/Object2FloatMap;
private final tickingSounds : Ljava/util/List;
private final queuedSounds : Ljava/util/Map;
private final soundDeleteTime : Ljava/util/Map;
private final listeners : Ljava/util/List;
private final queuedTickableSounds : Ljava/util/List;
private final preloadQueue : Ljava/util/List;
public <init>(Lnet/minecraft/client/sounds/SoundManager;Lnet/minecraft/client/Options;Lnet/minecraft/server/packs/resources/ResourceProvider;)V
public reload()V
private loadLibrary()V
public refreshCategoryVolume(Lnet/minecraft/sounds/SoundSource;)V
public destroy()V
public emergencyShutdown()V
public stop(Lnet/minecraft/client/resources/sounds/SoundInstance;)V
public updateCategoryVolume(Lnet/minecraft/sounds/SoundSource;F)V
public stopAll()V
public addEventListener(Lnet/minecraft/client/sounds/SoundEventListener;)V
public removeEventListener(Lnet/minecraft/client/sounds/SoundEventListener;)V
private shouldChangeDevice()Z
public tick(Z)V
private tickInGameSound()V
private notifyListeners(Lnet/minecraft/client/resources/sounds/SoundInstance;Lnet/minecraft/client/sounds/WeighedSoundEvents;F)V
private static getRange(ZLnet/minecraft/client/resources/sounds/SoundInstance$Attenuation;F)F
private tickMusicWhenPaused()V
private static requiresManualLooping(Lnet/minecraft/client/resources/sounds/SoundInstance;)Z
private static shouldLoopManually(Lnet/minecraft/client/resources/sounds/SoundInstance;)Z
private static shouldLoopAutomatically(Lnet/minecraft/client/resources/sounds/SoundInstance;)Z
public isActive(Lnet/minecraft/client/resources/sounds/SoundInstance;)Z
public play(Lnet/minecraft/client/resources/sounds/SoundInstance;)Lnet/minecraft/client/sounds/SoundEngine$PlayResult;
public queueTickingSound(Lnet/minecraft/client/resources/sounds/TickableSoundInstance;)V
public requestPreload(Lnet/minecraft/client/resources/sounds/Sound;)V
private calculatePitch(Lnet/minecraft/client/resources/sounds/SoundInstance;)F
private calculateVolume(Lnet/minecraft/client/resources/sounds/SoundInstance;)F
private calculateVolume(FLnet/minecraft/sounds/SoundSource;)F
public pauseAllExcept([Lnet/minecraft/sounds/SoundSource;)V
public resume()V
public playDelayed(Lnet/minecraft/client/resources/sounds/SoundInstance;I)V
public updateSource(Lnet/minecraft/client/Camera;)V
public stop(Lnet/minecraft/resources/Identifier;Lnet/minecraft/sounds/SoundSource;)V
public getChannelDebugString()Ljava/lang/String;
public getSoundCacheDebugStats(Lnet/minecraft/client/sounds/SoundBufferLibrary$DebugOutput;)V
public getAvailableSoundDevices()Ljava/util/List;
public getListenerTransform()Lcom/mojang/blaze3d/audio/ListenerTransform;
private synthetic lambda$updateSource$0(Lcom/mojang/blaze3d/audio/ListenerTransform;)V
private static synthetic lambda$resume$0(Ljava/util/stream/Stream;)V
private static synthetic lambda$play$3(Lnet/minecraft/client/sounds/ChannelAccess$ChannelHandle;Lnet/minecraft/client/sounds/AudioStream;)V
private static synthetic lambda$play$4(Lnet/minecraft/client/sounds/AudioStream;Lcom/mojang/blaze3d/audio/Channel;)V
private static synthetic lambda$play$1(Lnet/minecraft/client/sounds/ChannelAccess$ChannelHandle;Lcom/mojang/blaze3d/audio/SoundBuffer;)V
private static synthetic lambda$play$2(Lcom/mojang/blaze3d/audio/SoundBuffer;Lcom/mojang/blaze3d/audio/Channel;)V
private static synthetic lambda$play$0(FFLnet/minecraft/client/resources/sounds/SoundInstance;FZZLnet/minecraft/world/phys/Vec3;Lcom/mojang/blaze3d/audio/Channel;)V
private static synthetic lambda$tickInGameSound$0(FFLnet/minecraft/world/phys/Vec3;Lcom/mojang/blaze3d/audio/Channel;)V
private synthetic lambda$refreshCategoryVolume$0(Lnet/minecraft/sounds/SoundSource;Lnet/minecraft/client/resources/sounds/SoundInstance;Lnet/minecraft/client/sounds/ChannelAccess$ChannelHandle;)V
private static synthetic lambda$refreshCategoryVolume$1(FLcom/mojang/blaze3d/audio/Channel;)V
private static synthetic lambda$new$0(Lit/unimi/dsi/fastutil/objects/Object2FloatOpenHashMap;)V
static <clinit>()V
```
