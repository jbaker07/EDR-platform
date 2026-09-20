---
type: "event"
event: "net.fabricmc.fabric.api.event.lifecycle.v1.CommonLifecycleEvents.TAGS_LOADED"
callback: "net.fabricmc.fabric.api.event.lifecycle.v1.CommonLifecycleEvents$TagsLoaded"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.event.lifecycle.v1.CommonLifecycleEvents.TAGS_LOADED

Callback interface: `net.fabricmc.fabric.api.event.lifecycle.v1.CommonLifecycleEvents$TagsLoaded`

Module: [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]]

## Published from

| site | vanilla injection | environment | evidence |
|---|---|---|---|
| `ReloadableServerResourcesMixin.hookRefresh` | `ReloadableServerResources.updateComponentsAndStaticRegistryTags` @Inject at TAIL | both | static_inference |
| `ClientConfigurationPacketListenerImplMixin.invokeTagsLoaded` | `ClientConfigurationPacketListenerImpl.handleConfigurationFinished` @Inject at INVOKE Lnet/minecraft/network/Connection;setupInboundProtocol(Lnet/minecraft/network/ProtocolInfo;Lnet/minecraft/network/PacketListener;)V | client | static_inference |
| `ClientPacketListenerMixin.invokeTagsLoaded` | `ClientPacketListener.handleUpdateTags` @Inject at INVOKE Lnet/minecraft/world/item/CreativeModeTabs;searchTab()Lnet/minecraft/world/item/CreativeModeTab; | client | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
