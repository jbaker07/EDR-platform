---
type: "interface"
fqcn: "net.minecraft.server.level.ServerLevel$EntityCallbacks"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.level.ServerLevel$EntityCallbacks

System: [[20-Systems/net.minecraft.server.level|net.minecraft.server.level]]

`class` final; extends `java/lang/Object`; implements `net/minecraft/world/level/entity/LevelCallback`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `onTrackingEnd` | `(Lnet/minecraft/world/entity/Entity;)V` | exact | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `onTrackingStart` | `(Lnet/minecraft/world/entity/Entity;)V` | exact | @Inject at ['TAIL'] | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| reads | `this$0` | `Lnet/minecraft/server/level/ServerLevel;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | declared |

## Declared members (1 fields, 15 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
final synthetic this$0 : Lnet/minecraft/server/level/ServerLevel;
private <init>(Lnet/minecraft/server/level/ServerLevel;)V
public onCreated(Lnet/minecraft/world/entity/Entity;)V
public onDestroyed(Lnet/minecraft/world/entity/Entity;)V
public onTickingStart(Lnet/minecraft/world/entity/Entity;)V
public onTickingEnd(Lnet/minecraft/world/entity/Entity;)V
public onTrackingStart(Lnet/minecraft/world/entity/Entity;)V
public onTrackingEnd(Lnet/minecraft/world/entity/Entity;)V
public onSectionChange(Lnet/minecraft/world/entity/Entity;)V
public synthetic onSectionChange(Ljava/lang/Object;)V
public synthetic onTrackingEnd(Ljava/lang/Object;)V
public synthetic onTrackingStart(Ljava/lang/Object;)V
public synthetic onTickingEnd(Ljava/lang/Object;)V
public synthetic onTickingStart(Ljava/lang/Object;)V
public synthetic onDestroyed(Ljava/lang/Object;)V
public synthetic onCreated(Ljava/lang/Object;)V
```
