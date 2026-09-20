package com.example.rainlantern;

/**
 * The charging rules, with no Minecraft types in sight.
 *
 * <p>Everything here is a pure function of values the caller supplies. That is
 * deliberate: it is the part of the feature whose behaviour can be tested
 * without a game, and keeping it separable is what makes "charges in the rain
 * at the configured rate" a checkable claim rather than something only a
 * running server could settle.
 *
 * <p>Authored by hand. See PROVENANCE.md -- this is developer-authored
 * reference work, not ModCheck output.
 */
public final class LanternCharge {
    /** Ticks between charging steps. Game time, not wall-clock. */
    public static final int INTERVAL_TICKS = 20;

    /** Charge is capped so it serialises and renders predictably. */
    public static final int MAX_CHARGE = 1000;

    /** Bounds on the configured rate, in charge points per interval. */
    public static final int MIN_RATE = 0;
    public static final int MAX_RATE = 100;

    /** Default rate, used when the gamerule has never been set. */
    public static final int DEFAULT_RATE = 1;

    private LanternCharge() {
    }

    /**
     * Whether this game tick is a charging step.
     *
     * <p>Keyed on game time rather than a per-lantern counter so that every
     * lantern charges on the same schedule, and so a lantern that was unloaded
     * for a while does not resume mid-interval.
     */
    public static boolean isChargingTick(long gameTime, int intervalTicks) {
        if (intervalTicks <= 0) {
            // A non-positive interval would mean every tick, which is not a
            // schedule. Treated as "never" rather than as a busy loop.
            return false;
        }
        return Math.floorMod(gameTime, intervalTicks) == 0;
    }

    /** Clamp a configured rate into the supported range. */
    public static int clampRate(int rate) {
        return Math.max(MIN_RATE, Math.min(MAX_RATE, rate));
    }

    /**
     * The charge after one step.
     *
     * @param current the charge now
     * @param rate    charge points per interval, already clamped
     * @param exposed whether rain is falling on the lantern
     * @return the new charge, never above {@link #MAX_CHARGE}
     */
    public static int step(int current, int rate, boolean exposed) {
        if (!exposed) {
            // Sheltered lanterns hold their charge. They do not decay: the
            // request says nothing about decay, and inventing one would be a
            // behaviour nobody asked for.
            return clampCharge(current);
        }
        return clampCharge(clampCharge(current) + clampRate(rate));
    }

    /** Clamp a charge into the supported range, including a negative load. */
    public static int clampCharge(int charge) {
        return Math.max(0, Math.min(MAX_CHARGE, charge));
    }
}
