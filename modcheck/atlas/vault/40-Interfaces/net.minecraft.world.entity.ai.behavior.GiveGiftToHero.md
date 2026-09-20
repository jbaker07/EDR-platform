---
type: "interface"
fqcn: "net.minecraft.world.entity.ai.behavior.GiveGiftToHero"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity.ai.behavior.GiveGiftToHero

System: [[20-Systems/net.minecraft.world.entity|net.minecraft.world.entity]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `<clinit>` | `@Inject at TAIL` | both | [[30-Mechanisms/fabric-content-registries-v0|fabric-content-registries-v0]] | direct_reference |

## Declared members (29, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.entity.ai.behavior.GiveGiftToHero extends net.minecraft.world.entity.ai.behavior.Behavior<net.minecraft.world.entity.npc.villager.Villager> {
    private static final int THROW_GIFT_AT_DISTANCE;
    private static final int MIN_TIME_BETWEEN_GIFTS;
    private static final int MAX_TIME_BETWEEN_GIFTS;
    private static final int TIME_TO_DELAY_FOR_HEAD_TO_FINISH_TURNING;
    private static final java.util.Map<net.minecraft.resources.ResourceKey<net.minecraft.world.entity.npc.villager.VillagerProfession>, net.minecraft.resources.ResourceKey<net.minecraft.world.level.storage.loot.LootTable>> GIFTS;
    private static final float SPEED_MODIFIER;
    private int timeUntilNextGift;
    private boolean giftGivenDuringThisRun;
    private long timeSinceStart;
    public net.minecraft.world.entity.ai.behavior.GiveGiftToHero(int);
    protected boolean checkExtraStartConditions(net.minecraft.server.level.ServerLevel, net.minecraft.world.entity.npc.villager.Villager);
    protected void start(net.minecraft.server.level.ServerLevel, net.minecraft.world.entity.npc.villager.Villager, long);
    protected boolean canStillUse(net.minecraft.server.level.ServerLevel, net.minecraft.world.entity.npc.villager.Villager, long);
    protected void tick(net.minecraft.server.level.ServerLevel, net.minecraft.world.entity.npc.villager.Villager, long);
    protected void stop(net.minecraft.server.level.ServerLevel, net.minecraft.world.entity.npc.villager.Villager, long);
    private void throwGift(net.minecraft.server.level.ServerLevel, net.minecraft.world.entity.npc.villager.Villager, net.minecraft.world.entity.LivingEntity);
    private static net.minecraft.resources.ResourceKey<net.minecraft.world.level.storage.loot.LootTable> getLootTableToThrow(net.minecraft.world.entity.npc.villager.Villager);
    private boolean isHeroVisible(net.minecraft.world.entity.npc.villager.Villager);
    private java.util.Optional<net.minecraft.world.entity.player.Player> getNearestTargetableHero(net.minecraft.world.entity.npc.villager.Villager);
    private boolean isHero(net.minecraft.world.entity.player.Player);
    private boolean isWithinThrowingDistance(net.minecraft.world.entity.npc.villager.Villager, net.minecraft.world.entity.player.Player);
    private static int calculateTimeUntilNextGift(net.minecraft.server.level.ServerLevel);
    protected boolean checkExtraStartConditions(net.minecraft.server.level.ServerLevel, net.minecraft.world.entity.LivingEntity);
    protected boolean canStillUse(net.minecraft.server.level.ServerLevel, net.minecraft.world.entity.LivingEntity, long);
    protected void stop(net.minecraft.server.level.ServerLevel, net.minecraft.world.entity.LivingEntity, long);
    protected void tick(net.minecraft.server.level.ServerLevel, net.minecraft.world.entity.LivingEntity, long);
    protected void start(net.minecraft.server.level.ServerLevel, net.minecraft.world.entity.LivingEntity, long);
    private static void lambda$throwGift$0(net.minecraft.world.entity.npc.villager.Villager, net.minecraft.world.entity.LivingEntity, net.minecraft.server.level.ServerLevel, net.minecraft.world.item.ItemStack);
    static {};
}
```
