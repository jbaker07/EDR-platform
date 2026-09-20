---
type: "system"
package: "net.minecraft.client"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client

94 classes (41 top-level) across 1 packages in the processed jar; 3 changed by Loom processing; 12 hooked by Fabric API.

## Hooked types

- [[40-Interfaces/net.minecraft.client.CloudStatus|CloudStatus]] -- reads:1 -- by fabric-client-gametest-api-v1
- [[40-Interfaces/net.minecraft.client.GameNarrator|GameNarrator]] -- calls:1 -- by fabric-command-api-v2
- [[40-Interfaces/net.minecraft.client.InputType|InputType]] -- calls:1 -- by fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.client.KeyMapping|KeyMapping]] -- calls:6 -- by fabric-client-gametest-api-v1, fabric-events-interaction-v0, fabric-key-mapping-api-v1
- [[40-Interfaces/net.minecraft.client.KeyMapping_Category|KeyMapping$Category]] -- calls:6, injects_into:1, reads:1 -- by fabric-key-mapping-api-v1
- [[40-Interfaces/net.minecraft.client.KeyboardHandler|KeyboardHandler]] -- wraps:3 -- by fabric-screen-api-v1
- [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]] -- calls:132, injects_into:25, reads:117, wraps:1 -- by fabric-client-gametest-api-v1, fabric-command-api-v2, fabric-creative-tab-api-v1, fabric-data-attachment-api-v1, fabric-data-generation-api-v1, fabric-debug-api-v1, fabric-events-interaction-v0, fabric-game-rule-api-v1, fabric-item-api-v1, fabric-key-mapping-api-v1, fabric-lifecycle-events-v1, fabric-menu-api-v1, fabric-networking-api-v1, fabric-particles-v1, fabric-registry-sync-v0, fabric-renderer-api-v1, fabric-rendering-v1, fabric-resource-loader-v1, fabric-screen-api-v1, fabric-tag-api-v1, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.client.MouseHandler|MouseHandler]] -- calls:4, wraps:5 -- by fabric-client-gametest-api-v1, fabric-events-interaction-v0, fabric-screen-api-v1
- [[40-Interfaces/net.minecraft.client.OptionInstance|OptionInstance]] -- calls:9 -- by fabric-client-gametest-api-v1, fabric-renderer-api-v1
- [[40-Interfaces/net.minecraft.client.Options|Options]] -- calls:8, injects_into:3, reads:6, wraps:1, writes:2 -- by fabric-client-gametest-api-v1, fabric-events-interaction-v0, fabric-key-mapping-api-v1, fabric-renderer-api-v1, fabric-resource-loader-v1, fabric-transfer-api-v1
- [[40-Interfaces/net.minecraft.client.Options_3|Options$3]] -- injects_into:1 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.client.Screenshot|Screenshot]] -- calls:1 -- by fabric-client-gametest-api-v1

## Declared inventory

### `net.minecraft.client` (41 top-level)

`AttackIndicatorStatus`, `Camera`, `CameraType`, `ClientBootstrap`, `ClientBrandRetriever`, `ClientClockManager`, `ClientRecipeBook`, [[40-Interfaces/net.minecraft.client.CloudStatus|CloudStatus]], `CommandHistory`, `ComponentCollector`, `DebugQueryHandler`, `DeltaTracker`, `FramerateLimiter`, `GameLoadCookie`, [[40-Interfaces/net.minecraft.client.GameNarrator|GameNarrator]], `GraphicsPreset`, `HotbarManager`, `InactivityFpsLimit`, [[40-Interfaces/net.minecraft.client.InputType|InputType]], [[40-Interfaces/net.minecraft.client.KeyMapping|KeyMapping]], [[40-Interfaces/net.minecraft.client.KeyboardHandler|KeyboardHandler]], [[40-Interfaces/net.minecraft.client.Minecraft|Minecraft]], [[40-Interfaces/net.minecraft.client.MouseHandler|MouseHandler]], `MusicToastDisplayState`, `NarratorStatus`, [[40-Interfaces/net.minecraft.client.OptionInstance|OptionInstance]], [[40-Interfaces/net.minecraft.client.Options|Options]], `PeriodicNotificationManager`, `PreferredGraphicsApi`, `PresenceSharing`, `PrioritizeChunkUpdates`, `ResourceLoadStateTracker`, `RotatingSectionStorage`, [[40-Interfaces/net.minecraft.client.Screenshot|Screenshot]], `ScrollWheelHandler`, `SectionUpdateTracker`, `StringSplitter`, `TextureFilteringMethod`, `ToggleKeyMapping`, `User`, `package-info`

