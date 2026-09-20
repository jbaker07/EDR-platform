---
type: "event"
event: "net.fabricmc.fabric.api.block.v1.FluidFlowEvents.ALLOW"
callback: "net.fabricmc.fabric.api.block.v1.FluidFlowEvents$Allow"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.block.v1.FluidFlowEvents.ALLOW

Callback interface: `net.fabricmc.fabric.api.block.v1.FluidFlowEvents$Allow`

Module: [[30-Mechanisms/fabric-block-api-v1|fabric-block-api-v1]]

## Published from

| site | vanilla injection | environment | evidence |
|---|---|---|---|
| `FlowingFluidMixin.shouldSpreadLiquid` | `FlowingFluid.spreadTo` @Inject at HEAD | both | static_inference |
| `LavaFluidMixin.shouldSpreadLiquid` | `LavaFluid.spreadTo` @Inject at HEAD | both | static_inference |
| `LiquidBlockMixin.shouldSpreadLiquid` | `LiquidBlock.shouldSpreadLiquid` @Inject at HEAD | both | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. No analyst has stated one ([[_authored/contracts/_index|contracts]]); the callback signature above is all that is known.
