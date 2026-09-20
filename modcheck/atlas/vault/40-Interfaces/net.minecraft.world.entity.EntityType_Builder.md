---
type: "interface"
fqcn: "net.minecraft.world.entity.EntityType$Builder"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity.EntityType$Builder

System: [[20-Systems/net.minecraft.world.entity|net.minecraft.world.entity]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `build` | `@Inject at RETURN` | both | [[30-Mechanisms/fabric-object-builder-api-v1|fabric-object-builder-api-v1]] | direct_reference |

## Declared members (46, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.entity.EntityType$Builder<T extends net.minecraft.world.entity.Entity> {
    private final net.minecraft.world.entity.EntityType$EntityFactory<T> factory;
    private final net.minecraft.world.entity.MobCategory category;
    private net.minecraft.tags.TagKey<net.minecraft.world.level.block.Block> immuneTo;
    private boolean serialize;
    private boolean summon;
    private boolean fireImmune;
    private boolean canSpawnFarFromPlayer;
    private int clientTrackingRange;
    private int updateInterval;
    private net.minecraft.world.entity.EntityDimensions dimensions;
    private float spawnDimensionsScale;
    private net.minecraft.world.entity.EntityAttachments$Builder attachments;
    private net.minecraft.world.flag.FeatureFlagSet requiredFeatures;
    private net.minecraft.resources.DependantName<net.minecraft.world.entity.EntityType<?>, java.util.Optional<net.minecraft.resources.ResourceKey<net.minecraft.world.level.storage.loot.LootTable>>> lootTable;
    private final net.minecraft.resources.DependantName<net.minecraft.world.entity.EntityType<?>, java.lang.String> descriptionId;
    private boolean allowedInPeaceful;
    private boolean trackDeltas;
    private net.minecraft.world.entity.EntityType$Builder(net.minecraft.world.entity.EntityType$EntityFactory<T>, net.minecraft.world.entity.MobCategory);
    public static <T extends net.minecraft.world.entity.Entity> net.minecraft.world.entity.EntityType$Builder<T> of(net.minecraft.world.entity.EntityType$EntityFactory<T>, net.minecraft.world.entity.MobCategory);
    public static <T extends net.minecraft.world.entity.Entity> net.minecraft.world.entity.EntityType$Builder<T> createNothing(net.minecraft.world.entity.MobCategory);
    public net.minecraft.world.entity.EntityType$Builder<T> sized(float, float);
    public net.minecraft.world.entity.EntityType$Builder<T> spawnDimensionsScale(float);
    public net.minecraft.world.entity.EntityType$Builder<T> eyeHeight(float);
    public net.minecraft.world.entity.EntityType$Builder<T> passengerAttachments(float...);
    public net.minecraft.world.entity.EntityType$Builder<T> passengerAttachments(net.minecraft.world.phys.Vec3...);
    public net.minecraft.world.entity.EntityType$Builder<T> vehicleAttachment(net.minecraft.world.phys.Vec3);
    public net.minecraft.world.entity.EntityType$Builder<T> ridingOffset(float);
    public net.minecraft.world.entity.EntityType$Builder<T> nameTagOffset(float);
    public net.minecraft.world.entity.EntityType$Builder<T> attach(net.minecraft.world.entity.EntityAttachment, float, float, float);
    public net.minecraft.world.entity.EntityType$Builder<T> attach(net.minecraft.world.entity.EntityAttachment, net.minecraft.world.phys.Vec3);
    public net.minecraft.world.entity.EntityType$Builder<T> noSummon();
    public net.minecraft.world.entity.EntityType$Builder<T> noSave();
    public net.minecraft.world.entity.EntityType$Builder<T> fireImmune();
    public net.minecraft.world.entity.EntityType$Builder<T> immuneTo(net.minecraft.tags.TagKey<net.minecraft.world.level.block.Block>);
    public net.minecraft.world.entity.EntityType$Builder<T> canSpawnFarFromPlayer();
    public net.minecraft.world.entity.EntityType$Builder<T> clientTrackingRange(int);
    public net.minecraft.world.entity.EntityType$Builder<T> updateInterval(int);
    public net.minecraft.world.entity.EntityType$Builder<T> noUpdateInterval();
    public net.minecraft.world.entity.EntityType$Builder<T> requiredFeatures(net.minecraft.world.flag.FeatureFlag...);
    public net.minecraft.world.entity.EntityType$Builder<T> noLootTable();
    public net.minecraft.world.entity.EntityType$Builder<T> notInPeaceful();
    public net.minecraft.world.entity.EntityType$Builder<T> dontTrackDeltas();
    public net.minecraft.world.entity.EntityType<T> build(net.minecraft.resources.ResourceKey<net.minecraft.world.entity.EntityType<?>>);
    private static net.minecraft.world.entity.Entity lambda$createNothing$0(net.minecraft.world.entity.EntityType, net.minecraft.world.level.Level);
    private static java.lang.String lambda$new$1(net.minecraft.resources.ResourceKey);
    private static java.util.Optional lambda$new$0(net.minecraft.resources.ResourceKey);
}
```
