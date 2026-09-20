package com.example.rainlantern.client;

import com.example.rainlantern.ChargeCache;
import com.example.rainlantern.LanternChargePayload;
import net.fabricmc.api.ClientModInitializer;
import net.fabricmc.fabric.api.client.networking.v1.ClientPlayConnectionEvents;
import net.fabricmc.fabric.api.client.networking.v1.ClientPlayNetworking;

/**
 * The client half: receive charges, remember them, and draw them.
 *
 * <p>Everything in this package is reached only from the {@code client}
 * entrypoint. A dedicated server never loads it, which matters because it
 * touches rendering classes that do not exist there -- a class that merely
 * mentions them would crash the server at class-load time, long before any of
 * its code ran.
 *
 * <p>Authored by hand. See PROVENANCE.md.
 */
public class RainLanternClient implements ClientModInitializer {
    /** What the client knows. Single-threaded: written only from the client thread. */
    public static final ChargeCache CHARGES = new ChargeCache();

    @Override
    public void onInitializeClient() {
        ClientPlayNetworking.registerGlobalReceiver(
                LanternChargePayload.TYPE, (payload, context) ->
                        // The handler runs on the network thread. Touching game
                        // state from there races the client thread, so the work
                        // is handed over and only then does it touch the cache.
                        context.client().execute(() -> CHARGES.put(
                                payload.pos().asLong(), payload.charge(),
                                clientTick(context.client()))));

        // A charge from one world is not about another. Cleared on disconnect
        // rather than left to age out, so rejoining never shows the old world's
        // numbers.
        ClientPlayConnectionEvents.DISCONNECT.register((handler, client) -> CHARGES.clear());

        LanternHud.register();
    }

    /** The client's tick counter, used to age cache entries. */
    static long clientTick(net.minecraft.client.Minecraft client) {
        return client.level == null ? 0L : client.level.getGameTime();
    }
}
