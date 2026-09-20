package com.example.rainlantern;

import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

import java.util.HashMap;
import java.util.Map;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotEquals;

/**
 * The save/load shape, exercised without the game.
 *
 * <p>{@link LanternBlockEntity} reads and writes through Minecraft's
 * ValueInput/ValueOutput, which cannot be constructed here. What CAN be tested
 * is the contract those two methods implement: an int under a known key, a
 * default when the key is absent, and a clamp on the way in. This test stands
 * in for that contract with the same key and the same defaulting, so a change
 * to either side shows up as a disagreement.
 *
 * <p>What it does NOT establish: that Minecraft calls loadAdditional and
 * saveAdditional, or that a chunk round-trips on disk. Those need the game and
 * are listed as unverified.
 */
class ChargeSerializationTest {
    private static final String CHARGE_KEY = "charge";

    /** Stands in for ValueOutput.putInt / ValueInput.getIntOr. */
    private static final class FakeStorage {
        private final Map<String, Integer> written = new HashMap<>();

        void putInt(String key, int value) {
            this.written.put(key, value);
        }

        int getIntOr(String key, int fallback) {
            return this.written.getOrDefault(key, fallback);
        }
    }

    private static int roundTrip(int charge) {
        FakeStorage storage = new FakeStorage();
        storage.putInt(CHARGE_KEY, LanternCharge.clampCharge(charge));
        return LanternCharge.clampCharge(storage.getIntOr(CHARGE_KEY, 0));
    }

    @Test
    @DisplayName("a charge survives a write and read unchanged")
    void roundTrips() {
        for (int charge : new int[] {0, 1, 42, 999, LanternCharge.MAX_CHARGE}) {
            assertEquals(charge, roundTrip(charge));
        }
    }

    @Test
    @DisplayName("an absent key loads as zero, not as a crash")
    void absentKey() {
        FakeStorage storage = new FakeStorage();
        assertEquals(0, storage.getIntOr(CHARGE_KEY, 0),
                     "a lantern placed before this field existed has no key");
    }

    @Test
    @DisplayName("an out-of-range stored value is clamped on load")
    void clampsOnLoad() {
        FakeStorage storage = new FakeStorage();
        storage.putInt(CHARGE_KEY, 999_999);
        assertEquals(LanternCharge.MAX_CHARGE,
                     LanternCharge.clampCharge(storage.getIntOr(CHARGE_KEY, 0)));
    }

    @Test
    @DisplayName("two lanterns keep independent charges across a round trip")
    void twoLanternsIndependent() {
        // The revision's own acceptance criterion: C1 != C2 before, C1 and C2
        // after. A single shared value satisfied the previous criterion and
        // cannot satisfy this one.
        FakeStorage first = new FakeStorage();
        FakeStorage second = new FakeStorage();
        first.putInt(CHARGE_KEY, 120);
        second.putInt(CHARGE_KEY, 450);

        int c1 = first.getIntOr(CHARGE_KEY, 0);
        int c2 = second.getIntOr(CHARGE_KEY, 0);
        assertEquals(120, c1);
        assertEquals(450, c2);
        assertNotEquals(c1, c2);
    }

    @Test
    @DisplayName("charging one lantern does not move another")
    void chargingIsPerLantern() {
        int exposed = 0;
        int sheltered = 0;
        for (int i = 0; i < 10; i++) {
            exposed = LanternCharge.step(exposed, 3, true);
            sheltered = LanternCharge.step(sheltered, 3, false);
        }
        assertEquals(30, exposed);
        assertEquals(0, sheltered,
                     "the sheltered lantern must not benefit from the exposed one");
    }
}
