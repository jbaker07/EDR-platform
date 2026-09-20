package com.example.rainlantern.client;

import com.example.rainlantern.ChargeCache;
import com.example.rainlantern.LanternCharge;
import com.example.rainlantern.RainLanternContent;
import com.example.rainlantern.Rainlantern;
import net.fabricmc.fabric.api.client.rendering.v1.hud.HudElement;
import net.fabricmc.fabric.api.client.rendering.v1.hud.HudElementRegistry;
import net.fabricmc.fabric.api.client.rendering.v1.hud.VanillaHudElements;
import net.minecraft.client.DeltaTracker;
import net.minecraft.client.Minecraft;
import net.minecraft.client.gui.GuiGraphicsExtractor;
import net.minecraft.core.BlockPos;
import net.minecraft.network.chat.Component;
import net.minecraft.resources.Identifier;
import net.minecraft.world.level.Level;
import net.minecraft.world.phys.BlockHitResult;
import net.minecraft.world.phys.HitResult;

/**
 * The charge of the lantern the player is looking at.
 *
 * <p>Three ways this shows nothing, and they are deliberately the same
 * outcome: the player is not looking at a lantern, the client has never been
 * told that lantern's charge, or what it was told has aged out. Showing a
 * number in the last two would be showing something the client does not know.
 *
 * <p>The HUD interface in this Fabric API version is
 * {@code extractRenderState(GuiGraphicsExtractor, DeltaTracker)} -- there is no
 * {@code GuiGraphics} type in Minecraft 26.3 at all, and no
 * {@code HudRenderCallback}. Both were read from the resolved jars.
 *
 * <p>Authored by hand. See PROVENANCE.md.
 */
public final class LanternHud implements HudElement {
    private static final Identifier ELEMENT_ID =
            Identifier.fromNamespaceAndPath(Rainlantern.MOD_ID, "lantern_charge");

    /** Pixels above the bottom edge, clear of the hotbar. */
    private static final int BOTTOM_OFFSET = 60;

    private static final int WHITE = 0xFFFFFFFF;

    private LanternHud() {
    }

    public static void register() {
        // Attached after a named vanilla element rather than added last, so the
        // position does not depend on what other mods have registered.
        HudElementRegistry.attachElementAfter(
                VanillaHudElements.HOTBAR, ELEMENT_ID, new LanternHud());
    }

    @Override
    public void extractRenderState(GuiGraphicsExtractor graphics, DeltaTracker delta) {
        Minecraft client = Minecraft.getInstance();
        if (client.level == null || client.player == null) {
            return;
        }

        int charge = lookedAtCharge(client);
        if (charge == ChargeCache.UNKNOWN) {
            // Not looking at a lantern, never told, or told too long ago. The
            // HUD says nothing rather than something it cannot stand behind.
            return;
        }

        Component text = Component.literal(
                "Lantern charge: " + charge + " / " + LanternCharge.MAX_CHARGE);
        graphics.centeredText(client.font, text,
                              graphics.guiWidth() / 2,
                              graphics.guiHeight() - BOTTOM_OFFSET,
                              WHITE);
    }

    /**
     * The charge of the lantern under the crosshair, or {@link ChargeCache#UNKNOWN}.
     *
     * <p>Separated from rendering because it is the part the revision changed:
     * which lantern, rather than how it is drawn.
     */
    static int lookedAtCharge(Minecraft client) {
        BlockPos pos = lookedAtLantern(client);
        if (pos == null) {
            return ChargeCache.UNKNOWN;
        }
        return RainLanternClient.CHARGES.get(pos.asLong(),
                                             RainLanternClient.clientTick(client));
    }

    /** The position of the lantern under the crosshair, or null. */
    static BlockPos lookedAtLantern(Minecraft client) {
        HitResult hit = client.hitResult;
        if (!(hit instanceof BlockHitResult blockHit)
                || hit.getType() != HitResult.Type.BLOCK) {
            return null;
        }
        Level level = client.level;
        if (level == null) {
            return null;
        }
        BlockPos pos = blockHit.getBlockPos();
        if (!level.getBlockState(pos).is(RainLanternContent.LANTERN_BLOCK)) {
            // Looking at a block that is not one of ours.
            return null;
        }
        return pos;
    }
}
