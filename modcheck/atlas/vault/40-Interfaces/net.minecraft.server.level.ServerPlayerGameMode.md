---
type: "interface"
fqcn: "net.minecraft.server.level.ServerPlayerGameMode"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.level.ServerPlayerGameMode

System: [[20-Systems/net.minecraft.server.level|net.minecraft.server.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `destroyBlock` | `@Inject at INVOKE Lnet/minecraft/world/level/block/Block;playerWillDestroy(Lnet/` | both | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| injects_into | `destroyBlock` | `@Inject at INVOKE Lnet/minecraft/world/level/block/Block;destroy(Lnet/minecraft/` | both | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| injects_into | `handleBlockBreakAction` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| injects_into | `useItem` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |
| injects_into | `useItemOn` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |

## Declared members (36, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.server.level.ServerPlayerGameMode {
    private static final double FLIGHT_DISABLE_RANGE;
    private static final org.slf4j.Logger LOGGER;
    protected net.minecraft.server.level.ServerLevel level;
    protected final net.minecraft.server.level.ServerPlayer player;
    private net.minecraft.world.level.GameType gameModeForPlayer;
    private net.minecraft.world.level.GameType previousGameModeForPlayer;
    private boolean isDestroyingBlock;
    private int destroyProgressStart;
    private net.minecraft.core.BlockPos destroyPos;
    private net.minecraft.core.Direction destroyDirection;
    private int gameTicks;
    private boolean hasDelayedDestroy;
    private net.minecraft.core.BlockPos delayedDestroyPos;
    private int delayedTickStart;
    private int lastSentState;
    public net.minecraft.server.level.ServerPlayerGameMode(net.minecraft.server.level.ServerPlayer);
    public boolean changeGameModeForPlayer(net.minecraft.world.level.GameType);
    protected void setGameModeForPlayer(net.minecraft.world.level.GameType, net.minecraft.world.level.GameType);
    private boolean isInRangeOfGround();
    public net.minecraft.world.level.GameType getGameModeForPlayer();
    public net.minecraft.world.level.GameType getPreviousGameModeForPlayer();
    public boolean isSurvival();
    public boolean isCreative();
    public void tick();
    private float incrementDestroyProgress(net.minecraft.world.level.block.state.BlockState, net.minecraft.core.BlockPos, int);
    private void debugLogging(net.minecraft.core.BlockPos, boolean, int, java.lang.String);
    public void handleBlockBreakAction(net.minecraft.core.BlockPos, net.minecraft.network.protocol.game.ServerboundPlayerActionPacket$Action, net.minecraft.core.Direction, int, int);
    private void abortDestroyBlock(net.minecraft.core.BlockPos, int);
    public void destroyAndAck(net.minecraft.core.BlockPos, int, java.lang.String);
    public boolean destroyBlock(net.minecraft.core.BlockPos);
    public net.minecraft.world.InteractionResult useItem(net.minecraft.server.level.ServerPlayer, net.minecraft.world.level.Level, net.minecraft.world.item.ItemStack, net.minecraft.world.InteractionHand);
    public net.minecraft.world.InteractionResult useItemOn(net.minecraft.server.level.ServerPlayer, net.minecraft.world.level.Level, net.minecraft.world.item.ItemStack, net.minecraft.world.InteractionHand, net.minecraft.world.phys.BlockHitResult);
    public void setLevel(net.minecraft.server.level.ServerLevel);
    private static net.minecraft.world.item.ItemStack lambda$useItemOn$0(net.minecraft.server.level.ServerPlayer, net.minecraft.world.InteractionHand);
    private void lambda$handleBlockBreakAction$0(net.minecraft.world.item.ItemStack);
    static {};
}
```
