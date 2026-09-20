---
type: "interface"
fqcn: "net.minecraft.server.level.ServerEntity"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.level.ServerEntity

System: [[20-Systems/net.minecraft.server.level|net.minecraft.server.level]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `addPairing` | `(Lnet/minecraft/server/level/ServerPlayer;)V` | name_only | @Inject at ['TAIL'] | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `removePairing` | `(Lnet/minecraft/server/level/ServerPlayer;)V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| reads | `entity` | `Lnet/minecraft/world/entity/Entity;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | declared |

## Declared members (22 fields, 15 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final TOLERANCE_LEVEL_ROTATION : I
private static final TOLERANCE_LEVEL_POSITION : D
public static final FORCED_POS_UPDATE_PERIOD : I
private static final FORCED_TELEPORT_PERIOD : I
private final level : Lnet/minecraft/server/level/ServerLevel;
private final entity : Lnet/minecraft/world/entity/Entity;
private final updateInterval : Lnet/minecraft/world/entity/UpdateInterval;
private final trackDelta : Z
private final synchronizer : Lnet/minecraft/server/level/ServerEntity$Synchronizer;
private final interpolationTracker : Lnet/minecraft/world/entity/InterpolationTracker;
private final positionCodec : Lnet/minecraft/network/protocol/game/VecDeltaCodec;
private lastSentYRot : B
private lastSentXRot : B
private lastSentYHeadRot : B
private lastSentMovement : Lnet/minecraft/world/phys/Vec3;
private tickCount : I
private teleportDelay : I
private lastPassengers : Ljava/util/List;
private wasRiding : Z
private wasOnGround : Z
private trackedDataValues : Ljava/util/List;
public <init>(Lnet/minecraft/server/level/ServerLevel;Lnet/minecraft/world/entity/Entity;Lnet/minecraft/world/entity/UpdateInterval;ZLnet/minecraft/server/level/ServerEntity$Synchronizer;)V
public sendChanges()V
private createMovePacket(Lnet/minecraft/world/entity/PositionPath;BBZZ)Lnet/minecraft/network/protocol/game/MovementPacket;
private isFullPrecisionEncodingRequired(Lnet/minecraft/world/phys/Vec3;Lnet/minecraft/network/protocol/game/VecDelta;)Z
private handleMinecartPosRot(Lnet/minecraft/world/entity/vehicle/minecart/NewMinecartBehavior;BBZ)V
public removePairing(Lnet/minecraft/server/level/ServerPlayer;)V
public addPairing(Lnet/minecraft/server/level/ServerPlayer;)V
public sendPairingData(Lnet/minecraft/server/level/ServerPlayer;Ljava/util/function/Consumer;)V
public getPositionBase()Lnet/minecraft/world/phys/Vec3;
public getLastSentMovement()Lnet/minecraft/world/phys/Vec3;
public getLastSentXRot()F
public getLastSentYRot()F
public getLastSentYHeadRot()F
private sendDirtyEntityData()V
static <clinit>()V
```
