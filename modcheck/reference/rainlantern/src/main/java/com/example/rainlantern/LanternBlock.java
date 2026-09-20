package com.example.rainlantern;

import net.minecraft.core.BlockPos;
import net.minecraft.world.level.Level;
import net.minecraft.world.level.block.BaseEntityBlock;
import net.minecraft.world.level.block.entity.BlockEntity;
import net.minecraft.world.level.block.entity.BlockEntityTicker;
import net.minecraft.world.level.block.entity.BlockEntityType;
import net.minecraft.world.level.block.state.BlockState;

/**
 * The lantern itself.
 *
 * <p>Carries a block entity so each placement has its own charge, and a ticker
 * so the game drives the charging schedule. Using the game's own per-block
 * ticker rather than a level-wide tick handler is what removes the problem of
 * finding every lantern in a level: the game already knows where the loaded
 * ones are.
 *
 * <p>Minecraft 26.3 declares no block codec: BaseEntityBlock has no abstract
 * {@code codec()} and there is no {@code simpleCodec} helper, both of which
 * older versions required. Checked against the resolved jar rather than
 * carried over from a 1.21-era example.
 *
 * <p>Authored by hand. See PROVENANCE.md.
 */
public class LanternBlock extends BaseEntityBlock {
    public LanternBlock(Properties properties) {
        super(properties);
    }

    @Override
    public BlockEntity newBlockEntity(BlockPos pos, BlockState state) {
        return new LanternBlockEntity(pos, state);
    }

    @Override
    public <T extends BlockEntity> BlockEntityTicker<T> getTicker(
            Level level, BlockState state, BlockEntityType<T> type) {
        if (level.isClientSide()) {
            // The charge is server-owned. A client-side ticker would compute a
            // second, drifting copy of it.
            return null;
        }
        return createTickerHelper(type, RainLanternContent.LANTERN_BLOCK_ENTITY,
                                  LanternBlockEntity::serverTick);
    }
}
