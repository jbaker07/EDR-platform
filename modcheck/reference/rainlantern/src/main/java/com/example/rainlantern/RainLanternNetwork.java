package com.example.rainlantern;

import net.fabricmc.fabric.api.networking.v1.PayloadTypeRegistry;
import net.fabricmc.fabric.api.networking.v1.PlayerLookup;
import net.fabricmc.fabric.api.networking.v1.ServerPlayNetworking;
import net.minecraft.core.BlockPos;
import net.minecraft.server.level.ServerLevel;
import net.minecraft.server.level.ServerPlayer;
import net.minecraft.world.level.ChunkPos;

/**
 * Getting a server-side charge to the clients that can see the lantern.
 *
 * <p>A payload type is a definition; this is the synchronisation. Three parts
 * have to exist for the value to arrive: the type registered for the
 * server-to-client direction, a send with recipients, and -- on the other side
 * -- a receiver. Any one missing and the build still succeeds.
 *
 * <p>Authored by hand. See PROVENANCE.md.
 */
public final class RainLanternNetwork {
    private RainLanternNetwork() {
    }

    /**
     * Register the payload type for the server-to-client play phase.
     *
     * <p>Called from the MAIN entrypoint, not the client one: the type must be
     * known on a dedicated server too, or the send fails there. The client
     * registers its receiver separately, and both sides must agree on the type.
     */
    public static void registerCommon() {
        // clientboundPlay(), not playS2C(): the latter is the name in older
        // Fabric API versions and does not exist here.
        PayloadTypeRegistry.clientboundPlay().register(
                LanternChargePayload.TYPE, LanternChargePayload.CODEC);
    }

    /**
     * Tell the players who can see this lantern what its charge now is.
     *
     * <p>Recipients are the players tracking the lantern's chunk. Everyone else
     * cannot see it, and a player who walks into range gets the value from the
     * next change or from {@link #sendTo} on demand.
     *
     * <p>canSend is checked because a client without this mod has not
     * registered the type, and sending to it is an error rather than a no-op --
     * which matters on a server where the mod is not required of clients.
     */
    public static void broadcastCharge(ServerLevel level, BlockPos pos, int charge) {
        LanternChargePayload payload = new LanternChargePayload(pos, charge);
        // ChunkPos takes chunk coordinates; there is no BlockPos constructor
        // in this version, so the block position is converted explicitly.
        ChunkPos chunk = new ChunkPos(pos.getX() >> 4, pos.getZ() >> 4);
        for (ServerPlayer player : PlayerLookup.tracking(level, chunk)) {
            if (ServerPlayNetworking.canSend(player, LanternChargePayload.TYPE)) {
                ServerPlayNetworking.send(player, payload);
            }
        }
    }

    /** Send one lantern's charge to one player. */
    public static void sendTo(ServerPlayer player, BlockPos pos, int charge) {
        if (ServerPlayNetworking.canSend(player, LanternChargePayload.TYPE)) {
            ServerPlayNetworking.send(player, new LanternChargePayload(pos, charge));
        }
    }
}
