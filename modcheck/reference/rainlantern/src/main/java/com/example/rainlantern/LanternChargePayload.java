package com.example.rainlantern;

import net.minecraft.core.BlockPos;
import net.minecraft.network.RegistryFriendlyByteBuf;
import net.minecraft.network.codec.ByteBufCodecs;
import net.minecraft.network.codec.StreamCodec;
import net.minecraft.network.protocol.common.custom.CustomPacketPayload;
import net.minecraft.resources.Identifier;

/**
 * One lantern's charge, on the wire.
 *
 * <p>Carries the position as well as the value: the client caches per lantern,
 * and a value with no position could only ever describe one.
 *
 * <p>Authored by hand. See PROVENANCE.md.
 */
public record LanternChargePayload(BlockPos pos, int charge) implements CustomPacketPayload {
    public static final CustomPacketPayload.Type<LanternChargePayload> TYPE =
            new CustomPacketPayload.Type<>(Identifier.fromNamespaceAndPath(
                    Rainlantern.MOD_ID, "lantern_charge"));

    public static final StreamCodec<RegistryFriendlyByteBuf, LanternChargePayload> CODEC =
            StreamCodec.composite(
                    BlockPos.STREAM_CODEC, LanternChargePayload::pos,
                    ByteBufCodecs.VAR_INT, LanternChargePayload::charge,
                    LanternChargePayload::new);

    @Override
    public CustomPacketPayload.Type<LanternChargePayload> type() {
        return TYPE;
    }
}
