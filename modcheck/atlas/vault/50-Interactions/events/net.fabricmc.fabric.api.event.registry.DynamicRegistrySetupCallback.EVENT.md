---
type: "event"
event: "net.fabricmc.fabric.api.event.registry.DynamicRegistrySetupCallback.EVENT"
callback: "net.fabricmc.fabric.api.event.registry.DynamicRegistrySetupCallback"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.event.registry.DynamicRegistrySetupCallback.EVENT

Callback interface: `net.fabricmc.fabric.api.event.registry.DynamicRegistrySetupCallback`

Module: [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `RegistryDataLoaderMixin.beforeLoad` @101 | [[40-Interfaces/net.minecraft.resources.RegistryDataLoader|RegistryDataLoader]].`lambda$load$0` @WrapOperation INVOKE `Lnet/minecraft/resources/RegistryDataLoader;createContext(Ljava/util/List;Ljava/util/List;)Lnet/minecraft/resources/RegistryOps$RegistryInfoLookup;` | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
