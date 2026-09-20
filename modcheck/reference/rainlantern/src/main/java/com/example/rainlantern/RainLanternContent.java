package com.example.rainlantern;

import net.fabricmc.fabric.api.gamerule.v1.GameRuleBuilder;
import net.minecraft.core.Registry;
import net.minecraft.core.registries.BuiltInRegistries;
import net.minecraft.core.registries.Registries;
import net.minecraft.resources.Identifier;
import net.minecraft.resources.ResourceKey;
import net.minecraft.world.item.BlockItem;
import net.minecraft.world.item.Item;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.level.block.Block;
import net.minecraft.world.level.block.SoundType;
import net.minecraft.world.level.block.entity.BlockEntityType;
import net.minecraft.world.level.block.state.BlockBehaviour;
import net.minecraft.world.level.gamerules.GameRule;
import net.minecraft.world.level.gamerules.GameRuleCategory;
import net.minecraft.world.level.material.MapColor;

import java.util.Set;

/**
 * Everything the mod registers, and the single place registration happens.
 *
 * <p>All of it runs from {@link Rainlantern#onInitialize()}. Registration is
 * not idempotent in Minecraft -- registering the same id twice is a hard
 * failure, not a silent overwrite -- so it happens exactly once, from one
 * place, at the point the loader calls us.
 *
 * <p>Authored by hand. See PROVENANCE.md.
 */
public final class RainLanternContent {
    public static final Identifier LANTERN_ID =
            Identifier.fromNamespaceAndPath(Rainlantern.MOD_ID, "rain_lantern");

    public static final ResourceKey<Block> LANTERN_BLOCK_KEY =
            ResourceKey.create(Registries.BLOCK, LANTERN_ID);
    public static final ResourceKey<Item> LANTERN_ITEM_KEY =
            ResourceKey.create(Registries.ITEM, LANTERN_ID);

    public static final Block LANTERN_BLOCK = new LanternBlock(
            BlockBehaviour.Properties.of()
                    .mapColor(MapColor.METAL)
                    .strength(3.5F)
                    .sound(SoundType.LANTERN)
                    .lightLevel(state -> 15)
                    .setId(LANTERN_BLOCK_KEY));

    public static final Item LANTERN_ITEM = new BlockItem(
            LANTERN_BLOCK, new Item.Properties().setId(LANTERN_ITEM_KEY));

    public static final BlockEntityType<LanternBlockEntity> LANTERN_BLOCK_ENTITY =
            new BlockEntityType<>(LanternBlockEntity::new, Set.of(LANTERN_BLOCK));

    /**
     * Charge points added per charging interval, settable with
     * {@code /gamerule rainlantern:rain_lantern_charge_rate}.
     *
     * <p>Per interval rather than per tick or per second: ticks are not
     * wall-clock time on a loaded server, so an interval is the only stable
     * unit for this value.
     */
    public static final GameRule<Integer> CHARGE_RATE = GameRuleBuilder
            .forInteger(LanternCharge.DEFAULT_RATE)
            .category(GameRuleCategory.MISC)
            .buildAndRegister(Identifier.fromNamespaceAndPath(
                    Rainlantern.MOD_ID, "rain_lantern_charge_rate"));

    private RainLanternContent() {
    }

    /**
     * The configured rate for a level, clamped into the supported range.
     *
     * <p>Takes a ServerLevel because getGameRules() is declared there, not on
     * Level. That is also correct for this feature: the rate is read where the
     * charging happens, which is the server.
     */
    public static int chargeRate(ServerLevel level) {
        return LanternCharge.clampRate(level.getGameRules().get(CHARGE_RATE));
    }

    /** Called once, from the mod's main entrypoint. */
    public static void register() {
        Registry.register(BuiltInRegistries.BLOCK, LANTERN_BLOCK_KEY, LANTERN_BLOCK);
        Registry.register(BuiltInRegistries.ITEM, LANTERN_ITEM_KEY, LANTERN_ITEM);
        Registry.register(BuiltInRegistries.BLOCK_ENTITY_TYPE, LANTERN_ID,
                          LANTERN_BLOCK_ENTITY);
        // Referencing CHARGE_RATE below forces this class's static initialiser,
        // which is what registers the gamerule. Made explicit rather than left
        // to class-loading order.
        //
        // No creative-tab entry: the fabric item-group module is not on this
        // toolchain's classpath, and the request never asked for one. The item
        // is obtainable with /give and drops from the block.
        Rainlantern.LOGGER.info("registered {} (charge rate gamerule: {})",
                                LANTERN_ID, CHARGE_RATE);
    }
}
