---
type: "system_note"
id: "net.minecraft.client.gui"
side: "client"
---

> [!warning] Analyst-authored
> Interpretation, not extraction. Claims cite evidence ids; anything uncited is opinion. Reviewed corrections go into the store records or the extractors, never into a generated note.

# GUI: screens, HUD and widgets

Package `net.minecraft.client.gui` -- generated view: [[20-Systems/net.minecraft.client.gui|hooked types]]

**Responsibility.** Screens, the in-game HUD (Gui), widgets, layouts and the GUI render-state extraction introduced with the 26.x renderer. 53 hooked types.

**Side.** client

**Threads.** The client render thread; nothing here runs on a dedicated server.

## Extension points

- [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] provides HUD element registration; HudElement in 26.3 extracts render state (`capability/display_information.fabric_hud_element` records the corrected signature after a build failure).
- [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]] publishes [[50-Interactions/events/net.fabricmc.fabric.api.client.screen.v1.ScreenEvents.AFTER_INIT|AFTER_INIT]] and per-screen events for widgets and rendering.
- [[30-Mechanisms/fabric-key-mapping-api-v1|fabric-key-mapping-api-v1]] registers key bindings.

## Evidence

- `capability/display_information.fabric_hud_element`
- `extracted/edges.json#injects_into`

