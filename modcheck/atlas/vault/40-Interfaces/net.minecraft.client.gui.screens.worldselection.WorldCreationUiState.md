---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.worldselection.WorldCreationUiState"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.worldselection.WorldCreationUiState

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getGameRules` | `()Lnet/minecraft/world/level/gamerules/GameRules;` | exact | invokevirtual@48 in `TestWorldBuilderImpl.setConsistentSettings` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getGameRules` | `()Lnet/minecraft/world/level/gamerules/GameRules;` | exact | invokevirtual@63 in `TestWorldBuilderImpl.setConsistentSettings` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getGameRules` | `()Lnet/minecraft/world/level/gamerules/GameRules;` | exact | invokevirtual@78 in `TestWorldBuilderImpl.setConsistentSettings` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getGameRules` | `()Lnet/minecraft/world/level/gamerules/GameRules;` | exact | invokevirtual@93 in `TestWorldBuilderImpl.setConsistentSettings` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getSettings` | `()Lnet/minecraft/client/gui/screens/worldselection/WorldCreationContex` | exact | invokevirtual@1 in `TestWorldBuilderImpl.setConsistentSettings` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getTargetFolder` | `()Ljava/lang/String;` | exact | invokevirtual@94 in `TestWorldBuilderImpl.lambda$navigateCreateWorldScreen$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `setGenerateStructures` | `(Z)V` | exact | invokevirtual@44 in `TestWorldBuilderImpl.setConsistentSettings` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `setSeed` | `(Ljava/lang/String;)V` | exact | invokevirtual@39 in `TestWorldBuilderImpl.setConsistentSettings` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `setWorldType` | `(Lnet/minecraft/client/gui/screens/worldselection/WorldCreationUiState` | exact | invokevirtual@33 in `TestWorldBuilderImpl.setConsistentSettings` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (16 fields, 48 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final DEFAULT_WORLD_NAME : Lnet/minecraft/network/chat/Component;
private final listeners : Ljava/util/List;
private name : Ljava/lang/String;
private gameMode : Lnet/minecraft/client/gui/screens/worldselection/WorldCreationUiState$SelectedGameMode;
private difficulty : Lnet/minecraft/world/Difficulty;
private allowCommands : Ljava/lang/Boolean;
private seed : Ljava/lang/String;
private generateStructures : Z
private bonusChest : Z
private final savesFolder : Ljava/nio/file/Path;
private targetFolder : Ljava/lang/String;
private settings : Lnet/minecraft/client/gui/screens/worldselection/WorldCreationContext;
private worldType : Lnet/minecraft/client/gui/screens/worldselection/WorldCreationUiState$WorldTypeEntry;
private final normalPresetList : Ljava/util/List;
private final altPresetList : Ljava/util/List;
private gameRules : Lnet/minecraft/world/level/gamerules/GameRules;
public <init>(Ljava/nio/file/Path;Lnet/minecraft/client/gui/screens/worldselection/WorldCreationContext;Ljava/util/Optional;Ljava/util/OptionalLong;)V
public addListener(Ljava/util/function/Consumer;)V
public onChanged()V
public setName(Ljava/lang/String;)V
private findResultFolder(Ljava/lang/String;)Ljava/lang/String;
public getName()Ljava/lang/String;
public getTargetFolder()Ljava/lang/String;
public setGameMode(Lnet/minecraft/client/gui/screens/worldselection/WorldCreationUiState$SelectedGameMode;)V
public getGameMode()Lnet/minecraft/client/gui/screens/worldselection/WorldCreationUiState$SelectedGameMode;
public setDifficulty(Lnet/minecraft/world/Difficulty;)V
public getDifficulty()Lnet/minecraft/world/Difficulty;
public isHardcore()Z
public setAllowCommands(Z)V
public isAllowCommands()Z
public setSeed(Ljava/lang/String;)V
public getSeed()Ljava/lang/String;
public setGenerateStructures(Z)V
public isGenerateStructures()Z
public setBonusChest(Z)V
public isBonusChest()Z
public setSettings(Lnet/minecraft/client/gui/screens/worldselection/WorldCreationContext;)V
public getSettings()Lnet/minecraft/client/gui/screens/worldselection/WorldCreationContext;
public updateDimensions(Lnet/minecraft/client/gui/screens/worldselection/WorldCreationContext$DimensionsUpdater;)V
protected tryUpdateDataConfiguration(Lnet/minecraft/world/level/WorldDataConfiguration;)Z
public isDebug()Z
public setWorldType(Lnet/minecraft/client/gui/screens/worldselection/WorldCreationUiState$WorldTypeEntry;)V
public getWorldType()Lnet/minecraft/client/gui/screens/worldselection/WorldCreationUiState$WorldTypeEntry;
public getPresetEditor()Lnet/minecraft/client/gui/screens/worldselection/PresetEditor;
public getNormalPresetList()Ljava/util/List;
public getAltPresetList()Ljava/util/List;
private updatePresetLists()V
private static findPreset(Lnet/minecraft/client/gui/screens/worldselection/WorldCreationContext;Ljava/util/Optional;)Ljava/util/Optional;
private static getNonEmptyList(Lnet/minecraft/core/Registry;Lnet/minecraft/tags/TagKey;)Ljava/util/Optional;
public setGameRules(Lnet/minecraft/world/level/gamerules/GameRules;)V
public getGameRules()Lnet/minecraft/world/level/gamerules/GameRules;
private static synthetic lambda$getNonEmptyList$1(Ljava/util/List;)Z
private static synthetic lambda$getNonEmptyList$0(Lnet/minecraft/core/HolderSet$Named;)Ljava/util/List;
private static synthetic lambda$findPreset$0(Lnet/minecraft/client/gui/screens/worldselection/WorldCreationContext;Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;
private static synthetic lambda$updatePresetLists$0(Lnet/minecraft/core/Registry;)Ljava/util/List;
private static synthetic lambda$setWorldType$0(Lnet/minecraft/core/Holder;Lnet/minecraft/core/RegistryAccess$Frozen;Lnet/minecraft/world/level/levelgen/WorldDimensions;)Lnet/minecraft/world/level/levelgen/WorldDimensions;
private synthetic lambda$setSeed$0(Lnet/minecraft/world/level/levelgen/WorldOptions;)Lnet/minecraft/world/level/levelgen/WorldOptions;
private static synthetic lambda$onChanged$1(ZLnet/minecraft/world/level/levelgen/WorldOptions;)Lnet/minecraft/world/level/levelgen/WorldOptions;
private static synthetic lambda$onChanged$0(ZLnet/minecraft/world/level/levelgen/WorldOptions;)Lnet/minecraft/world/level/levelgen/WorldOptions;
private synthetic lambda$new$3(Lnet/minecraft/world/level/levelgen/flat/FlatLevelGeneratorSettings;)V
private static synthetic lambda$new$2(Lnet/minecraft/core/Holder$Reference;)Lnet/minecraft/world/level/levelgen/flat/FlatLevelGeneratorSettings;
private static synthetic lambda$new$0(Lnet/minecraft/client/gui/screens/worldselection/WorldCreationContext;Lnet/minecraft/resources/ResourceKey;)Ljava/util/Optional;
private static synthetic lambda$new$1(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/core/Registry;)Ljava/util/Optional;
static <clinit>()V
```
