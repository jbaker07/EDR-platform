---
type: "interface"
fqcn: "net.minecraft.client.multiplayer.ClientDebugSubscriber"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.multiplayer.ClientDebugSubscriber

System: [[20-Systems/net.minecraft.client.multiplayer|net.minecraft.client.multiplayer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `requestedSubscriptions` | `@Inject at RETURN` | client | [[30-Mechanisms/fabric-debug-api-v1|fabric-debug-api-v1]] | direct_reference |

## Declared members (32, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.multiplayer.ClientDebugSubscriber {
    private final net.minecraft.client.multiplayer.ClientPacketListener connection;
    private final net.minecraft.client.gui.components.DebugScreenOverlay debugScreenOverlay;
    private java.util.Set<net.minecraft.util.debug.DebugSubscription<?>> remoteSubscriptions;
    private final java.util.Map<net.minecraft.util.debug.DebugSubscription<?>, net.minecraft.client.multiplayer.ClientDebugSubscriber$ValueMaps<?>> valuesBySubscription;
    public net.minecraft.client.multiplayer.ClientDebugSubscriber(net.minecraft.client.multiplayer.ClientPacketListener, net.minecraft.client.gui.components.DebugScreenOverlay);
    private static void addFlag(java.util.Set<net.minecraft.util.debug.DebugSubscription<?>>, net.minecraft.util.debug.DebugSubscription<?>, boolean);
    private java.util.Set<net.minecraft.util.debug.DebugSubscription<?>> requestedSubscriptions();
    public void clear();
    public void tick(long);
    private void onSubscriptionsChanged(java.util.Set<net.minecraft.util.debug.DebugSubscription<?>>);
    private void initializeSubscriptions(java.util.Set<net.minecraft.util.debug.DebugSubscription<?>>);
    private <V> net.minecraft.client.multiplayer.ClientDebugSubscriber$ValueMaps<V> getValueMaps(net.minecraft.util.debug.DebugSubscription<V>);
    private <K, V> net.minecraft.client.multiplayer.ClientDebugSubscriber$ValueMap<K, V> getValueMap(net.minecraft.util.debug.DebugSubscription<V>, net.minecraft.client.multiplayer.ClientDebugSubscriber$ValueMapType<K, V>);
    private <K, V> V getValue(net.minecraft.util.debug.DebugSubscription<V>, K, net.minecraft.client.multiplayer.ClientDebugSubscriber$ValueMapType<K, V>);
    public net.minecraft.util.debug.DebugValueAccess createDebugValueAccess(net.minecraft.world.level.Level);
    public <T> void updateChunk(long, net.minecraft.world.level.ChunkPos, net.minecraft.util.debug.DebugSubscription$Update<T>);
    public <T> void updateBlock(long, net.minecraft.core.BlockPos, net.minecraft.util.debug.DebugSubscription$Update<T>);
    public <T> void updateEntity(long, net.minecraft.world.entity.Entity, net.minecraft.util.debug.DebugSubscription$Update<T>);
    public <T> void pushEvent(long, net.minecraft.util.debug.DebugSubscription$Event<T>);
    private <K, V> void updateMap(long, K, net.minecraft.util.debug.DebugSubscription$Update<V>, net.minecraft.client.multiplayer.ClientDebugSubscriber$ValueMapType<K, V>);
    private <K, V> void forEachValue(net.minecraft.util.debug.DebugSubscription<V>, net.minecraft.client.multiplayer.ClientDebugSubscriber$ValueMapType<K, V>, java.util.function.BiConsumer<K, V>);
    public void dropLevel();
    public void dropChunk(net.minecraft.world.level.ChunkPos);
    public void dropEntity(net.minecraft.world.entity.Entity);
    private static <T> net.minecraft.client.multiplayer.ClientDebugSubscriber$ValueMapType<java.util.UUID, T> entities();
    private static <T> net.minecraft.client.multiplayer.ClientDebugSubscriber$ValueMapType<net.minecraft.core.BlockPos, T> blocks();
    private static <T> net.minecraft.client.multiplayer.ClientDebugSubscriber$ValueMapType<net.minecraft.world.level.ChunkPos, T> chunks();
    private static net.minecraft.client.multiplayer.ClientDebugSubscriber$ValueMap lambda$chunks$0(net.minecraft.client.multiplayer.ClientDebugSubscriber$ValueMaps);
    private static net.minecraft.client.multiplayer.ClientDebugSubscriber$ValueMap lambda$blocks$0(net.minecraft.client.multiplayer.ClientDebugSubscriber$ValueMaps);
    private static net.minecraft.client.multiplayer.ClientDebugSubscriber$ValueMap lambda$entities$0(net.minecraft.client.multiplayer.ClientDebugSubscriber$ValueMaps);
    private static net.minecraft.client.multiplayer.ClientDebugSubscriber$ValueMaps lambda$initializeSubscriptions$0(net.minecraft.util.debug.DebugSubscription);
    private static void lambda$tick$0(long, net.minecraft.util.debug.DebugSubscription, net.minecraft.client.multiplayer.ClientDebugSubscriber$ValueMaps);
}
```
