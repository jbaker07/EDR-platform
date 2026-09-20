package com.example.rainlantern;

import java.util.HashMap;
import java.util.Map;

/**
 * What the client knows about lantern charges, and for how long.
 *
 * <p>The client has no access to the server's stored charge; it knows only what
 * it was last told. That makes staleness a real concern rather than a
 * theoretical one: a lantern whose chunk the player has left, or whose update
 * was missed, would otherwise render a number the server changed long ago.
 *
 * <p>Keyed by packed block position so this class needs no Minecraft types and
 * can be tested directly. Not thread-safe: the client game thread is the only
 * writer, and the network handler hands work to it rather than writing here.
 *
 * <p>Authored by hand. See PROVENANCE.md.
 */
public final class ChargeCache {
    /** An entry older than this is treated as unknown. */
    public static final long MAX_AGE_TICKS = 200L;

    /** Returned when nothing is known about a position. */
    public static final int UNKNOWN = -1;

    private final Map<Long, Entry> entries = new HashMap<>();
    private final long maxAgeTicks;

    public ChargeCache() {
        this(MAX_AGE_TICKS);
    }

    public ChargeCache(long maxAgeTicks) {
        this.maxAgeTicks = maxAgeTicks;
    }

    private record Entry(int charge, long receivedAtTick) {
    }

    /** Record what the server said about a position, at the given client tick. */
    public void put(long packedPos, int charge, long nowTick) {
        this.entries.put(packedPos, new Entry(LanternCharge.clampCharge(charge), nowTick));
    }

    /**
     * The charge at a position, or {@link #UNKNOWN}.
     *
     * <p>Unknown covers three cases the HUD must treat identically: never told,
     * told too long ago, and explicitly forgotten. Showing a number in any of
     * them would be showing something we do not know.
     */
    public int get(long packedPos, long nowTick) {
        Entry entry = this.entries.get(packedPos);
        if (entry == null) {
            return UNKNOWN;
        }
        if (isStale(entry.receivedAtTick(), nowTick, this.maxAgeTicks)) {
            return UNKNOWN;
        }
        return entry.charge();
    }

    /** Drop everything. Used on disconnect: another world's charges are not ours. */
    public void clear() {
        this.entries.clear();
    }

    /** Drop entries that have aged out, so a long session does not grow forever. */
    public int pruneStale(long nowTick) {
        int before = this.entries.size();
        this.entries.entrySet().removeIf(
                e -> isStale(e.getValue().receivedAtTick(), nowTick, this.maxAgeTicks));
        return before - this.entries.size();
    }

    public int size() {
        return this.entries.size();
    }

    /**
     * Whether an entry received at one tick is stale at another.
     *
     * <p>A negative age -- an entry from the future -- happens when the client
     * tick counter resets on a world change. Treated as stale, because an entry
     * from a world we have left is not about this one.
     */
    public static boolean isStale(long receivedAtTick, long nowTick, long maxAgeTicks) {
        long age = nowTick - receivedAtTick;
        return age < 0 || age > maxAgeTicks;
    }
}
