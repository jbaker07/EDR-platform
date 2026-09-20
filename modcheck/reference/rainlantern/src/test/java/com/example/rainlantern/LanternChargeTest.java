package com.example.rainlantern;

import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertFalse;
import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * The charging rules, tested as behaviour.
 *
 * <p>These run without Minecraft. They establish that the rules are right; they
 * establish nothing about whether the game calls them, which is a separate
 * claim needing a game.
 */
class LanternChargeTest {

    // -- rain versus shelter -------------------------------------------------

    @Test
    @DisplayName("an exposed lantern gains exactly the configured rate")
    void exposedGainsRate() {
        assertEquals(5, LanternCharge.step(0, 5, true));
        assertEquals(12, LanternCharge.step(7, 5, true));
    }

    @Test
    @DisplayName("a sheltered lantern gains nothing and loses nothing")
    void shelteredHolds() {
        assertEquals(7, LanternCharge.step(7, 5, false));
        assertEquals(0, LanternCharge.step(0, 5, false));
    }

    @Test
    @DisplayName("charge does not decay when sheltered")
    void noDecay() {
        int charge = 100;
        for (int i = 0; i < 50; i++) {
            charge = LanternCharge.step(charge, 5, false);
        }
        assertEquals(100, charge, "decay is a behaviour nobody asked for");
    }

    // -- the schedule --------------------------------------------------------

    @Test
    @DisplayName("charging happens once per interval, not every tick")
    void schedule() {
        int charging = 0;
        for (long tick = 0; tick < 200; tick++) {
            if (LanternCharge.isChargingTick(tick, LanternCharge.INTERVAL_TICKS)) {
                charging++;
            }
        }
        assertEquals(10, charging, "200 ticks at one step per 20 is ten steps");
    }

    @Test
    @DisplayName("the schedule is keyed on game time, so every lantern shares it")
    void scheduleIsGlobal() {
        for (long tick : new long[] {0, 20, 40, 1000}) {
            assertTrue(LanternCharge.isChargingTick(tick, 20));
        }
        for (long tick : new long[] {1, 19, 21, 999}) {
            assertFalse(LanternCharge.isChargingTick(tick, 20));
        }
    }

    @Test
    @DisplayName("a negative game time still yields a stable schedule")
    void negativeGameTime() {
        // floorMod, not %, so -20 is a charging tick rather than -0.
        assertTrue(LanternCharge.isChargingTick(-20, 20));
        assertFalse(LanternCharge.isChargingTick(-19, 20));
    }

    @Test
    @DisplayName("a non-positive interval means never, not every tick")
    void degenerateInterval() {
        assertFalse(LanternCharge.isChargingTick(0, 0));
        assertFalse(LanternCharge.isChargingTick(100, -5));
    }

    // -- configuration boundaries -------------------------------------------

    @Test
    @DisplayName("the rate is clamped into the supported range")
    void rateBounds() {
        assertEquals(LanternCharge.MIN_RATE, LanternCharge.clampRate(-1));
        assertEquals(LanternCharge.MIN_RATE, LanternCharge.clampRate(Integer.MIN_VALUE));
        assertEquals(LanternCharge.MAX_RATE, LanternCharge.clampRate(Integer.MAX_VALUE));
        assertEquals(7, LanternCharge.clampRate(7));
    }

    @Test
    @DisplayName("a rate of zero charges nothing even in the rain")
    void zeroRate() {
        assertEquals(4, LanternCharge.step(4, 0, true));
    }

    @Test
    @DisplayName("an out-of-range rate is clamped by step, not trusted")
    void stepClampsRate() {
        assertEquals(LanternCharge.MAX_RATE, LanternCharge.step(0, 10_000, true));
        assertEquals(3, LanternCharge.step(3, -10, true));
    }

    // -- the cap -------------------------------------------------------------

    @Test
    @DisplayName("charge stops at the maximum and does not wrap")
    void capped() {
        assertEquals(LanternCharge.MAX_CHARGE,
                     LanternCharge.step(LanternCharge.MAX_CHARGE, 50, true));
        assertEquals(LanternCharge.MAX_CHARGE,
                     LanternCharge.step(LanternCharge.MAX_CHARGE - 1, 50, true));
    }

    @Test
    @DisplayName("a corrupt stored value is clamped on the way in")
    void clampsCorruptInput() {
        assertEquals(0, LanternCharge.clampCharge(-500));
        assertEquals(LanternCharge.MAX_CHARGE, LanternCharge.clampCharge(999_999));
        assertEquals(0, LanternCharge.step(-500, 0, false));
    }

    @Test
    @DisplayName("reaching the cap takes the number of intervals the rate implies")
    void timeToFull() {
        int charge = 0;
        int intervals = 0;
        while (charge < LanternCharge.MAX_CHARGE && intervals < 10_000) {
            charge = LanternCharge.step(charge, 10, true);
            intervals++;
        }
        assertEquals(LanternCharge.MAX_CHARGE, charge);
        assertEquals(100, intervals, "1000 points at 10 per interval");
    }
}
