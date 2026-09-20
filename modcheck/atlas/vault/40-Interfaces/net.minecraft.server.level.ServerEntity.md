---
type: "interface"
fqcn: "net.minecraft.server.level.ServerEntity"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.level.ServerEntity

System: [[20-Systems/net.minecraft.server.level|net.minecraft.server.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `addPairing` | `@Inject at TAIL` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `removePairing` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (37, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.server.level.ServerEntity {
    private static final org.slf4j.Logger LOGGER;
    private static final int TOLERANCE_LEVEL_ROTATION;
    private static final double TOLERANCE_LEVEL_POSITION;
    public static final int FORCED_POS_UPDATE_PERIOD;
    private static final int FORCED_TELEPORT_PERIOD;
    private final net.minecraft.server.level.ServerLevel level;
    private final net.minecraft.world.entity.Entity entity;
    private final net.minecraft.world.entity.UpdateInterval updateInterval;
    private final boolean trackDelta;
    private final net.minecraft.server.level.ServerEntity$Synchronizer synchronizer;
    private final net.minecraft.world.entity.InterpolationTracker interpolationTracker;
    private final net.minecraft.network.protocol.game.VecDeltaCodec positionCodec;
    private byte lastSentYRot;
    private byte lastSentXRot;
    private byte lastSentYHeadRot;
    private net.minecraft.world.phys.Vec3 lastSentMovement;
    private int tickCount;
    private int teleportDelay;
    private java.util.List<net.minecraft.world.entity.Entity> lastPassengers;
    private boolean wasRiding;
    private boolean wasOnGround;
    private java.util.List<net.minecraft.network.syncher.SynchedEntityData$DataValue<?>> trackedDataValues;
    public net.minecraft.server.level.ServerEntity(net.minecraft.server.level.ServerLevel, net.minecraft.world.entity.Entity, net.minecraft.world.entity.UpdateInterval, boolean, net.minecraft.server.level.ServerEntity$Synchronizer);
    public void sendChanges();
    private net.minecraft.network.protocol.game.MovementPacket<net.minecraft.network.protocol.game.ClientGamePacketListener> createMovePacket(net.minecraft.world.entity.PositionPath, byte, byte, boolean, boolean);
    private boolean isFullPrecisionEncodingRequired(net.minecraft.world.phys.Vec3, net.minecraft.network.protocol.game.VecDelta);
    private void handleMinecartPosRot(net.minecraft.world.entity.vehicle.minecart.NewMinecartBehavior, byte, byte, boolean);
    public void removePairing(net.minecraft.server.level.ServerPlayer);
    public void addPairing(net.minecraft.server.level.ServerPlayer);
    public void sendPairingData(net.minecraft.server.level.ServerPlayer, java.util.function.Consumer<net.minecraft.network.protocol.Packet<net.minecraft.network.protocol.game.ClientGamePacketListener>>);
    public net.minecraft.world.phys.Vec3 getPositionBase();
    public net.minecraft.world.phys.Vec3 getLastSentMovement();
    public float getLastSentXRot();
    public float getLastSentYRot();
    public float getLastSentYHeadRot();
    private void sendDirtyEntityData();
    static {};
}
```
