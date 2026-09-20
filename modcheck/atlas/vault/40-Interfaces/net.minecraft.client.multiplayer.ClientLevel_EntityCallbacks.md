---
type: "interface"
fqcn: "net.minecraft.client.multiplayer.ClientLevel$EntityCallbacks"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.multiplayer.ClientLevel$EntityCallbacks

System: [[20-Systems/net.minecraft.client.multiplayer|net.minecraft.client.multiplayer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `onTrackingEnd(Lnet/minecraft/world/entity/Entity;)V` | `@Inject at HEAD` | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `onTrackingStart(Lnet/minecraft/world/entity/Entity;)V` | `@Inject at TAIL` | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |

## Declared members (16, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
final class net.minecraft.client.multiplayer.ClientLevel$EntityCallbacks implements net.minecraft.world.level.entity.LevelCallback<net.minecraft.world.entity.Entity> {
    final net.minecraft.client.multiplayer.ClientLevel this$0;
    private net.minecraft.client.multiplayer.ClientLevel$EntityCallbacks(net.minecraft.client.multiplayer.ClientLevel);
    public void onCreated(net.minecraft.world.entity.Entity);
    public void onDestroyed(net.minecraft.world.entity.Entity);
    public void onTickingStart(net.minecraft.world.entity.Entity);
    public void onTickingEnd(net.minecraft.world.entity.Entity);
    public void onTrackingStart(net.minecraft.world.entity.Entity);
    public void onTrackingEnd(net.minecraft.world.entity.Entity);
    public void onSectionChange(net.minecraft.world.entity.Entity);
    public void onSectionChange(java.lang.Object);
    public void onTrackingEnd(java.lang.Object);
    public void onTrackingStart(java.lang.Object);
    public void onTickingEnd(java.lang.Object);
    public void onTickingStart(java.lang.Object);
    public void onDestroyed(java.lang.Object);
    public void onCreated(java.lang.Object);
}
```
