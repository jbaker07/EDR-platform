package com.example.rainlantern;

import net.fabricmc.api.ModInitializer;
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;

/**
 * The mod's main entrypoint: where every server-and-client registration happens.
 *
 * <p>This is the connection that makes the rest of the mod exist. A registration
 * method that nothing calls is dead code the compiler is perfectly happy with,
 * and the symptom in game is a mod that loads and does nothing.
 *
 * <p>Authored by hand. See PROVENANCE.md.
 */
public class Rainlantern implements ModInitializer {
    public static final String MOD_ID = "rainlantern";
    public static final Logger LOGGER = LoggerFactory.getLogger(MOD_ID);

    @Override
    public void onInitialize() {
        // Order matters: the payload type must be registered before anything
        // can send it, and content registration must finish before a world
        // loads. Both happen here, once, during mod initialisation.
        RainLanternNetwork.registerCommon();
        RainLanternContent.register();
        LOGGER.info("Rainlantern initialised");
    }
}
