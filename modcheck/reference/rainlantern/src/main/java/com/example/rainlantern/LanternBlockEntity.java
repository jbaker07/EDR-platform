package com.example.rainlantern;

import net.minecraft.core.BlockPos;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.world.level.Level;
import net.minecraft.world.level.block.entity.BlockEntity;
import net.minecraft.world.level.block.state.BlockState;
import net.minecraft.world.level.storage.ValueInput;
import net.minecraft.world.level.storage.ValueOutput;

/**
 * One lantern's own charge.
 *
 * <p>A block entity, not a level-wide store and not a data attachment. Each
 * placement has its own instance, the game writes it into the chunk, and the
 * charge is therefore per lantern by construction rather than by bookkeeping.
 * That is the requirement the revision added, and this is the simplest
 * mechanism that meets it: no extra API module, no registry of positions, and
 * no risk of the map and the world disagreeing about which lanterns exist.
 *
 * <p>Authored by hand. See PROVENANCE.md.
 */
public class LanternBlockEntity extends BlockEntity {
    private static final String CHARGE_KEY = "charge";

    private int charge;

    public LanternBlockEntity(BlockPos pos, BlockState state) {
        super(RainLanternContent.LANTERN_BLOCK_ENTITY, pos, state);
    }

    public int getCharge() {
        return this.charge;
    }

    /**
     * Set the charge, and tell the game to write the chunk.
     *
     * <p>Without setChanged the value lives in memory and is never saved, which
     * looks exactly like a reload bug and produces no error anywhere.
     */
    public void setCharge(int value) {
        int clamped = LanternCharge.clampCharge(value);
        if (clamped == this.charge) {
            return;
        }
        this.charge = clamped;
        this.setChanged();
    }

    @Override
    protected void loadAdditional(ValueInput input) {
        super.loadAdditional(input);
        // A missing key means a lantern placed before this field existed, or a
        // freshly placed one. Zero is the right answer for both.
        this.charge = LanternCharge.clampCharge(input.getIntOr(CHARGE_KEY, 0));
    }

    @Override
    protected void saveAdditional(ValueOutput output) {
        super.saveAdditional(output);
        output.putInt(CHARGE_KEY, this.charge);
    }

    /**
     * One charging step, called by the game for each loaded lantern.
     *
     * <p>Server-side only: the ticker is registered only for the server level,
     * because the charge is server-owned and a client-side increment would
     * drift from it.
     */
    public static void serverTick(Level level, BlockPos pos, BlockState state,
                                  LanternBlockEntity lantern) {
        if (!(level instanceof ServerLevel serverLevel)) {
            return;
        }
        if (!LanternCharge.isChargingTick(serverLevel.getGameTime(),
                                          LanternCharge.INTERVAL_TICKS)) {
            return;
        }

        // isRainingAt already accounts for sky visibility, the heightmap and
        // whether this biome has rain at its temperature -- a lantern in a
        // desert or under a slab is not exposed. Testing the position ABOVE the
        // lantern is what makes a slab overhead count as shelter.
        boolean exposed = serverLevel.isRainingAt(pos.above());

        int rate = RainLanternContent.chargeRate(serverLevel);
        int before = lantern.getCharge();
        int after = LanternCharge.step(before, rate, exposed);
        if (after == before) {
            return;
        }
        lantern.setCharge(after);
        // Only on an actual change: the HUD needs the current value, not a
        // packet every interval for every lantern in the world.
        RainLanternNetwork.broadcastCharge(serverLevel, pos, after);
    }
}
