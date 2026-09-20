package com.example.rainlantern;

import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertNotEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * What the client knows, and when it should admit it does not know.
 *
 * <p>The revision's display requirement turns on this: looking at lantern A and
 * then lantern B must show two different numbers, and looking at one the client
 * was never told about must show none.
 */
class ChargeCacheTest {
    private static final long A = 1L;
    private static final long B = 2L;

    // -- independent values per lantern -------------------------------------

    @Test
    @DisplayName("two lanterns hold independent values")
    void independent() {
        ChargeCache cache = new ChargeCache();
        cache.put(A, 10, 0);
        cache.put(B, 250, 0);
        assertEquals(10, cache.get(A, 0));
        assertEquals(250, cache.get(B, 0));
        assertNotEquals(cache.get(A, 0), cache.get(B, 0),
                        "a single shared value is exactly what the revision forbids");
    }

    @Test
    @DisplayName("switching the looked-at lantern switches the value")
    void switchTarget() {
        ChargeCache cache = new ChargeCache();
        cache.put(A, 10, 0);
        cache.put(B, 250, 0);
        long looking = A;
        assertEquals(10, cache.get(looking, 0));
        looking = B;
        assertEquals(250, cache.get(looking, 0));
    }

    @Test
    @DisplayName("updating one lantern leaves the other alone")
    void updateIsolated() {
        ChargeCache cache = new ChargeCache();
        cache.put(A, 10, 0);
        cache.put(B, 250, 0);
        cache.put(A, 11, 1);
        assertEquals(11, cache.get(A, 1));
        assertEquals(250, cache.get(B, 1));
    }

    // -- missing and stale state --------------------------------------------

    @Test
    @DisplayName("a lantern we were never told about is unknown, not zero")
    void neverTold() {
        assertEquals(ChargeCache.UNKNOWN, new ChargeCache().get(A, 0));
    }

    @Test
    @DisplayName("an entry older than the limit is unknown")
    void ages() {
        ChargeCache cache = new ChargeCache(200);
        cache.put(A, 42, 0);
        assertEquals(42, cache.get(A, 200), "still fresh at exactly the limit");
        assertEquals(ChargeCache.UNKNOWN, cache.get(A, 201));
    }

    @Test
    @DisplayName("an entry from the future is treated as stale")
    void futureEntry() {
        // The client tick counter resets on a world change; an entry that now
        // looks like it arrived in the future is from a world we have left.
        ChargeCache cache = new ChargeCache(200);
        cache.put(A, 42, 5_000);
        assertEquals(ChargeCache.UNKNOWN, cache.get(A, 10));
    }

    @Test
    @DisplayName("staleness is decided by age, in both directions")
    void stalenessRule() {
        assertFalse(ChargeCache.isStale(0, 0, 200));
        assertFalse(ChargeCache.isStale(0, 200, 200));
        assertTrue(ChargeCache.isStale(0, 201, 200));
        assertTrue(ChargeCache.isStale(100, 50, 200));
    }

    @Test
    @DisplayName("a fresh update revives a lantern that had aged out")
    void refresh() {
        ChargeCache cache = new ChargeCache(200);
        cache.put(A, 42, 0);
        assertEquals(ChargeCache.UNKNOWN, cache.get(A, 500));
        cache.put(A, 43, 500);
        assertEquals(43, cache.get(A, 500));
    }

    // -- housekeeping --------------------------------------------------------

    @Test
    @DisplayName("disconnecting forgets everything")
    void clearOnDisconnect() {
        ChargeCache cache = new ChargeCache();
        cache.put(A, 10, 0);
        cache.put(B, 20, 0);
        cache.clear();
        assertEquals(0, cache.size());
        assertEquals(ChargeCache.UNKNOWN, cache.get(A, 0));
    }

    @Test
    @DisplayName("pruning drops aged entries and keeps fresh ones")
    void prune() {
        ChargeCache cache = new ChargeCache(200);
        cache.put(A, 10, 0);
        cache.put(B, 20, 300);
        assertEquals(1, cache.pruneStale(300), "only the old one goes");
        assertEquals(1, cache.size());
        assertEquals(20, cache.get(B, 300));
    }

    @Test
    @DisplayName("a received value is clamped, so a bad packet cannot poison the HUD")
    void clampsOnPut() {
        ChargeCache cache = new ChargeCache();
        cache.put(A, 999_999, 0);
        assertEquals(LanternCharge.MAX_CHARGE, cache.get(A, 0));
        cache.put(B, -5, 0);
        assertEquals(0, cache.get(B, 0));
    }
}
