---
type: "mechanism"
module: "fabric-creative-tab-api-v1"
version: "5.0.21+fcdff87f5d"
sha256: "415e659be69014edac017bc384a96c4f2da5ce8fa2093e7d488593e05f03a0b6"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-creative-tab-api-v1

**Version** `5.0.21+fcdff87f5d` -- **artifact sha256** `415e659be69014edac017bc384a96c4f2da5ce8fa2093e7d488593e05f03a0b6`

## Declared (fabric.mod.json)

- environment: `*`
- depends: `{"fabricloader": ">=0.19.3", "fabric-api-base": "*", "fabric-resource-loader-v1": "*"}`
- entrypoints: `null`
- mixin configs: `["fabric-creative-tab-api-v1.mixins.json", {"config": "fabric-creative-tab-api-v1.client.mixins.json", "environment": "client"}]`
- access widener: `fabric-creative-tab-api-v1.classtweaker`

## Events this module publishes

- [[50-Interactions/events/net.fabricmc.fabric.api.creativetab.v1.CreativeModeTabEvents.MODIFY_OUTPUT_ALL|CreativeModeTabEvents.MODIFY_OUTPUT_ALL]]

## Vanilla types this module modifies (mixins)

| vanilla type | method | how | environment | mixin |
|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.gui.screens.inventory.CreativeModeInventoryScreen|CreativeModeInventoryScreen]] | `checkTabClicked` | injects_into `@Inject at HEAD` | client | `CreativeModeInventoryScreenMixin.isClickInTab` |
| [[40-Interfaces/net.minecraft.client.gui.screens.inventory.CreativeModeInventoryScreen|CreativeModeInventoryScreen]] | `checkTabHovering` | injects_into `@Inject at HEAD` | client | `CreativeModeInventoryScreenMixin.renderTabTooltipIfHovered` |
| [[40-Interfaces/net.minecraft.client.gui.screens.inventory.CreativeModeInventoryScreen|CreativeModeInventoryScreen]] | `extractTabButton` | injects_into `@Inject at HEAD` | client | `CreativeModeInventoryScreenMixin.extractTabButton` |
| [[40-Interfaces/net.minecraft.client.gui.screens.inventory.CreativeModeInventoryScreen|CreativeModeInventoryScreen]] | `init` | injects_into `@Inject at INVOKE Lnet/minecraft/client/gui/components/EditBox;setTextColor(I)V` | client | `CreativeModeInventoryScreenMixin.init` |
| [[40-Interfaces/net.minecraft.client.gui.screens.inventory.CreativeModeInventoryScreen|CreativeModeInventoryScreen]] | `keyPressed` | injects_into `@Inject at HEAD` | client | `CreativeModeInventoryScreenMixin.keyPressed` |
| [[40-Interfaces/net.minecraft.client.gui.screens.inventory.CreativeModeInventoryScreen|CreativeModeInventoryScreen]] | `selectTab` | injects_into `@Inject at HEAD` | client | `CreativeModeInventoryScreenMixin.setSelectedTab` |
| [[40-Interfaces/net.minecraft.world.item.CreativeModeTab|CreativeModeTab]] | `buildContents` | injects_into `@Inject at TAIL` | both | `CreativeModeTabMixin.getStacks` |
| [[40-Interfaces/net.minecraft.world.item.CreativeModeTabs|CreativeModeTabs]] | `buildAllTabContents` | injects_into `@Inject at TAIL` | both | `CreativeModeTabsMixin.paginateTabs` |
| [[40-Interfaces/net.minecraft.world.item.CreativeModeTabs|CreativeModeTabs]] | `validate` | injects_into `@Inject at HEAD` | both | `CreativeModeTabsMixin.deferDuplicateCheck` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.client.creativetab.v1.FabricCreativeModeInventoryScreen|FabricCreativeModeInventoryScreen]] (interface, 10 members)
- [[40-Interfaces/net.fabricmc.fabric.api.creativetab.v1.CreativeModeTabEvents|CreativeModeTabEvents]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.creativetab.v1.FabricCreativeModeTab|FabricCreativeModeTab]] (class, 1 members)
- [[40-Interfaces/net.fabricmc.fabric.api.creativetab.v1.FabricCreativeModeTabOutput|FabricCreativeModeTabOutput]] (class, 29 members)

## What this establishes, and does not

- Injection targets and API signatures are `direct_reference`: read from the jar.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- Nothing here is `observed`. No game ran.
