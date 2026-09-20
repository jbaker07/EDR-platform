---
type: "interface"
fqcn: "net.minecraft.client.sounds.SoundEngine"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.sounds.SoundEngine

System: [[20-Systems/net.minecraft.client.sounds|net.minecraft.client.sounds]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| wraps | `play(Lnet/minecraft/client/resources/sounds/SoundInstance;)Lnet/minecraft/client/sounds/SoundEngine$PlayResult;` | `@Redirect at INVOKE Lnet/minecraft/client/sounds/SoundBufferLibrary;getStream(Ln` | client | [[30-Mechanisms/fabric-sound-api-v1|fabric-sound-api-v1]] | direct_reference |

## Declared members (80, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.sounds.SoundEngine {
    private static final org.slf4j.Marker MARKER;
    private static final org.slf4j.Logger LOGGER;
    private static final float PITCH_MIN;
    private static final float PITCH_MAX;
    private static final float VOLUME_MIN;
    private static final float VOLUME_MAX;
    private static final int MIN_SOURCE_LIFETIME;
    private static final java.util.Set<net.minecraft.resources.Identifier> ONLY_WARN_ONCE;
    public static final java.lang.String MISSING_SOUND;
    public static final int LOOPING_SOUND_SUBTITLE_INTERVAL_TICKS;
    public static final java.lang.String OPEN_AL_SOFT_PREFIX;
    public static final int OPEN_AL_SOFT_PREFIX_LENGTH;
    private final net.minecraft.client.sounds.SoundManager soundManager;
    private final net.minecraft.client.Options options;
    private boolean loaded;
    private final com.mojang.blaze3d.audio.Library library;
    private final com.mojang.blaze3d.audio.Listener listener;
    private final net.minecraft.client.sounds.SoundBufferLibrary soundBuffers;
    private final net.minecraft.client.sounds.SoundEngineExecutor executor;
    private final net.minecraft.client.sounds.ChannelAccess channelAccess;
    private int tickCount;
    private com.mojang.blaze3d.audio.DeviceList lastSeenDevices;
    private final com.mojang.blaze3d.audio.DeviceTracker deviceTracker;
    private final java.util.Map<net.minecraft.client.resources.sounds.SoundInstance, net.minecraft.client.sounds.ChannelAccess$ChannelHandle> instanceToChannel;
    private final com.google.common.collect.Multimap<net.minecraft.sounds.SoundSource, net.minecraft.client.resources.sounds.SoundInstance> instanceBySource;
    private final it.unimi.dsi.fastutil.objects.Object2FloatMap<net.minecraft.sounds.SoundSource> gainBySource;
    private final java.util.List<net.minecraft.client.resources.sounds.TickableSoundInstance> tickingSounds;
    private final java.util.Map<net.minecraft.client.resources.sounds.SoundInstance, java.lang.Integer> queuedSounds;
    private final java.util.Map<net.minecraft.client.resources.sounds.SoundInstance, java.lang.Integer> soundDeleteTime;
    private final java.util.List<net.minecraft.client.sounds.SoundEventListener> listeners;
    private final java.util.List<net.minecraft.client.resources.sounds.TickableSoundInstance> queuedTickableSounds;
    private final java.util.List<net.minecraft.client.resources.sounds.Sound> preloadQueue;
    public net.minecraft.client.sounds.SoundEngine(net.minecraft.client.sounds.SoundManager, net.minecraft.client.Options, net.minecraft.server.packs.resources.ResourceProvider);
    public void reload();
    private synchronized void loadLibrary();
    public void refreshCategoryVolume(net.minecraft.sounds.SoundSource);
    public void destroy();
    public void emergencyShutdown();
    public void stop(net.minecraft.client.resources.sounds.SoundInstance);
    public void updateCategoryVolume(net.minecraft.sounds.SoundSource, float);
    public void stopAll();
    public void addEventListener(net.minecraft.client.sounds.SoundEventListener);
    public void removeEventListener(net.minecraft.client.sounds.SoundEventListener);
    private boolean shouldChangeDevice();
    public void tick(boolean);
    private void tickInGameSound();
    private void notifyListeners(net.minecraft.client.resources.sounds.SoundInstance, net.minecraft.client.sounds.WeighedSoundEvents, float);
    private static float getRange(boolean, net.minecraft.client.resources.sounds.SoundInstance$Attenuation, float);
    private void tickMusicWhenPaused();
    private static boolean requiresManualLooping(net.minecraft.client.resources.sounds.SoundInstance);
    private static boolean shouldLoopManually(net.minecraft.client.resources.sounds.SoundInstance);
    private static boolean shouldLoopAutomatically(net.minecraft.client.resources.sounds.SoundInstance);
    public boolean isActive(net.minecraft.client.resources.sounds.SoundInstance);
    public net.minecraft.client.sounds.SoundEngine$PlayResult play(net.minecraft.client.resources.sounds.SoundInstance);
    public void queueTickingSound(net.minecraft.client.resources.sounds.TickableSoundInstance);
    public void requestPreload(net.minecraft.client.resources.sounds.Sound);
    private float calculatePitch(net.minecraft.client.resources.sounds.SoundInstance);
    private float calculateVolume(net.minecraft.client.resources.sounds.SoundInstance);
    private float calculateVolume(float, net.minecraft.sounds.SoundSource);
    public void pauseAllExcept(net.minecraft.sounds.SoundSource...);
    public void resume();
    public void playDelayed(net.minecraft.client.resources.sounds.SoundInstance, int);
    public void updateSource(net.minecraft.client.Camera);
    public void stop(net.minecraft.resources.Identifier, net.minecraft.sounds.SoundSource);
    public java.lang.String getChannelDebugString();
    public void getSoundCacheDebugStats(net.minecraft.client.sounds.SoundBufferLibrary$DebugOutput);
    public java.util.List<java.lang.String> getAvailableSoundDevices();
    public com.mojang.blaze3d.audio.ListenerTransform getListenerTransform();
    private void lambda$updateSource$0(com.mojang.blaze3d.audio.ListenerTransform);
    private static void lambda$resume$0(java.util.stream.Stream);
    private static void lambda$play$3(net.minecraft.client.sounds.ChannelAccess$ChannelHandle, net.minecraft.client.sounds.AudioStream);
    private static void lambda$play$4(net.minecraft.client.sounds.AudioStream, com.mojang.blaze3d.audio.Channel);
    private static void lambda$play$1(net.minecraft.client.sounds.ChannelAccess$ChannelHandle, com.mojang.blaze3d.audio.SoundBuffer);
    private static void lambda$play$2(com.mojang.blaze3d.audio.SoundBuffer, com.mojang.blaze3d.audio.Channel);
    private static void lambda$play$0(float, float, net.minecraft.client.resources.sounds.SoundInstance, float, boolean, boolean, net.minecraft.world.phys.Vec3, com.mojang.blaze3d.audio.Channel);
    private static void lambda$tickInGameSound$0(float, float, net.minecraft.world.phys.Vec3, com.mojang.blaze3d.audio.Channel);
    private void lambda$refreshCategoryVolume$0(net.minecraft.sounds.SoundSource, net.minecraft.client.resources.sounds.SoundInstance, net.minecraft.client.sounds.ChannelAccess$ChannelHandle);
    private static void lambda$refreshCategoryVolume$1(float, com.mojang.blaze3d.audio.Channel);
    private static void lambda$new$0(it.unimi.dsi.fastutil.objects.Object2FloatOpenHashMap);
    static {};
}
```
