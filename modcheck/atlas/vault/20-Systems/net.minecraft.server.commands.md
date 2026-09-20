---
type: "system"
package: "net.minecraft.server.commands"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.commands

180 classes (111 top-level) across 3 packages in the processed jar; 0 changed by Loom processing; 6 hooked by Fabric API.

## Hooked types

- [[40-Interfaces/net.minecraft.server.commands.DataPackCommand|DataPackCommand]] -- injects_into:1, wraps:2 -- by fabric-resource-loader-v1
- [[40-Interfaces/net.minecraft.server.commands.DebugConfigCommand|DebugConfigCommand]] -- calls:1, wraps:1 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.server.commands.EnchantCommand|EnchantCommand]] -- wraps:1 -- by fabric-item-api-v1
- [[40-Interfaces/net.minecraft.server.commands.GameRuleCommand_1|GameRuleCommand$1]] -- injects_into:1, reads:1 -- by fabric-game-rule-api-v1
- [[40-Interfaces/net.minecraft.server.commands.data.BlockDataAccessor|BlockDataAccessor]] -- wraps:1 -- by fabric-data-attachment-api-v1
- [[40-Interfaces/net.minecraft.server.commands.data.EntityDataAccessor|EntityDataAccessor]] -- wraps:1 -- by fabric-data-attachment-api-v1

## Declared inventory

### `net.minecraft.server.commands` (100 top-level)

`AdvancementCommands`, `ArgProvider`, `AttributeCommand`, `BanIpCommands`, `BanListCommands`, `BanPlayerCommands`, `BossBarCommands`, `ChaseCommand`, `ClearInventoryCommands`, `CloneCommands`, `CommandResponseTracker`, `ComputeCommand`, `DamageCommand`, [[40-Interfaces/net.minecraft.server.commands.DataPackCommand|DataPackCommand]], `DeOpCommands`, `DebugCommand`, [[40-Interfaces/net.minecraft.server.commands.DebugConfigCommand|DebugConfigCommand]], `DebugMobSpawningCommand`, `DebugPathCommand`, `DefaultGameModeCommands`, `DialogCommand`, `DifficultyCommand`, `EffectCommands`, `EmoteCommands`, [[40-Interfaces/net.minecraft.server.commands.EnchantCommand|EnchantCommand]], `ExecuteCommand`, `ExperienceCommand`, `FetchProfileCommand`, `FillBiomeCommand`, `FillCommand`, `ForceLoadCommand`, `FunctionCommand`, `GameModeCommand`, `GameRuleCommand`, `GiveCommand`, `HelpCommand`, `InCommandFunction`, `JfrCommand`, `KickCommand`, `KillCommand`, `ListPlayersCommand`, `LocateCommand`, `LookAt`, `LootCommand`, `LootContextSources`, `MsgCommand`, `OpCommand`, `PardonCommand`, `PardonIpCommand`, `ParticleCommand`, `PerfCommand`, `PlaceCommand`, `PlaySoundCommand`, `PostEffectCommand`, `PublishCommand`, `RaidCommand`, `RandomCommand`, `RecipeCommand`, `ReloadCommand`, `ReturnCommand`, `RideCommand`, `RotateCommand`, `SaveAllCommand`, `SaveOffCommand`, `SaveOnCommand`, `SayCommand`, `ScheduleCommand`, `ScoreboardCommand`, `SeedCommand`, `ServerPackCommand`, `SetBlockCommand`, `SetPlayerIdleTimeoutCommand`, `SetSpawnCommand`, `SetWorldSpawnCommand`, `SpawnArmorTrimsCommand`, `SpectateCommand`, `SpreadPlayersCommand`, `StopCommand`, `StopSoundCommand`, `StopwatchCommand`, `SummonCommand`, `SwingCommand`, `TagCommand`, `TeamCommand`, `TeamMsgCommand`, `TeleportCommand`, `TellRawCommand`, `TickCommand`, `TimeCommand`, `TitleCommand`, `TransferCommand`, `TriggerCommand`, `UnpublishCommand`, `VersionCommand`, `WardenSpawnTrackerCommand`, `WaypointCommand`, `WeatherCommand`, `WhitelistCommand`, `WorldBorderCommand`, `package-info`

### `net.minecraft.server.commands.data` (6 top-level)

[[40-Interfaces/net.minecraft.server.commands.data.BlockDataAccessor|BlockDataAccessor]], `DataAccessor`, `DataCommands`, [[40-Interfaces/net.minecraft.server.commands.data.EntityDataAccessor|EntityDataAccessor]], `StorageDataAccessor`, `package-info`

### `net.minecraft.server.commands.item` (5 top-level)

`BlockItemAccessor`, `EntityItemAccessor`, `ItemAccessor`, `ItemCommands`, `package-info`

