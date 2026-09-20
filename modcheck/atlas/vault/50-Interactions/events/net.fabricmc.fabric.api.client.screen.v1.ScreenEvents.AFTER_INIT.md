---
type: "event"
event: "net.fabricmc.fabric.api.client.screen.v1.ScreenEvents.AFTER_INIT"
callback: "net.fabricmc.fabric.api.client.screen.v1.ScreenEvents$AfterInit"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.client.screen.v1.ScreenEvents.AFTER_INIT

Callback interface: `net.fabricmc.fabric.api.client.screen.v1.ScreenEvents$AfterInit`

Module: [[30-Mechanisms/fabric-screen-api-v1|fabric-screen-api-v1]]

## Published from

| site | vanilla injection (handler's own injects/wraps edges) | environment | evidence |
|---|---|---|---|
| `ScreenMixin.afterInit` @18 | (handler is not itself an injector: fired from a helper or impl method) | unknown | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
