---
type: "event"
event: "net.fabricmc.fabric.api.command.v2.CommandRegistrationCallback.EVENT"
callback: "net.fabricmc.fabric.api.command.v2.CommandRegistrationCallback"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.command.v2.CommandRegistrationCallback.EVENT

Callback interface: `net.fabricmc.fabric.api.command.v2.CommandRegistrationCallback`

Module: [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]]

## Published from

| site | vanilla injection | environment | evidence |
|---|---|---|---|
| `CommandsMixin.fabric_addCommands` | `Commands.<init>` @Inject at INVOKE Lcom/mojang/brigadier/CommandDispatcher;setConsumer(Lcom/mojang/brigadier/ResultConsumer;)V | both | static_inference |

## Contract

What a subscriber may assume is NOT established by extraction. An analyst-stated contract exists: [[_authored/contracts/net.fabricmc.fabric.api.command.v2.CommandRegistrationCallback.EVENT|read it]].
