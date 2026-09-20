---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.worldselection.CreateWorldScreen"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.worldselection.CreateWorldScreen

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`class` public; extends `net/minecraft/client/gui/screens/Screen`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getUiState` | `()Lnet/minecraft/client/gui/screens/worldselection/WorldCreationUiStat` | exact | invokevirtual@57 in `TestWorldBuilderImpl.lambda$navigateCreateWorldScreen$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `openFresh` | `(Lnet/minecraft/client/Minecraft;Ljava/lang/Runnable;)V` | exact | invokestatic@16 in `TestWorldBuilderImpl.lambda$navigateCreateWorldScreen$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `getDataPackSelectionSettings` | `(Lnet/minecraft/world/level/WorldDataConfiguration;)Lcom/mojang/datafi` | name_only | @Inject at ['INVOKE'] | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| injects_into | `onCreate` | `()V` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `openCreateWorldScreen` | `(Lnet/minecraft/client/Minecraft;Ljava/lang/Runnable;Ljava/util/functi` | exact | @ModifyVariable at ['INVOKE'] | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `tempDataPackRepository` | `Lnet/minecraft/server/packs/repository/PackRepository;` | exact | @Shadow declaration | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | declared |
| wraps | `openCreateWorldScreen` | `(Lnet/minecraft/client/Minecraft;Ljava/lang/Runnable;Ljava/util/functi` | exact | @Redirect at ['FIELD'] | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (22 fields, 71 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final GROUP_BOTTOM : I
private static final TAB_COLUMN_WIDTH : I
private static final LOGGER : Lorg/slf4j/Logger;
private static final TEMP_WORLD_PREFIX : Ljava/lang/String;
private static final GAME_MODEL_LABEL : Lnet/minecraft/network/chat/Component;
private static final NAME_LABEL : Lnet/minecraft/network/chat/Component;
private static final EXPERIMENTS_LABEL : Lnet/minecraft/network/chat/Component;
private static final ALLOW_COMMANDS_INFO : Lnet/minecraft/network/chat/Component;
private static final PREPARING_WORLD_DATA : Lnet/minecraft/network/chat/Component;
private static final HORIZONTAL_BUTTON_SPACING : I
private static final VERTICAL_BUTTON_SPACING : I
public static final TAB_HEADER_BACKGROUND : Lnet/minecraft/resources/Identifier;
private final layout : Lnet/minecraft/client/gui/layouts/HeaderAndFooterLayout;
private final uiState : Lnet/minecraft/client/gui/screens/worldselection/WorldCreationUiState;
private final tabManager : Lnet/minecraft/client/gui/components/tabs/TabManager;
private recreated : Z
private final packValidator : Lnet/minecraft/world/level/validation/DirectoryValidator;
private final createWorldCallback : Lnet/minecraft/client/gui/screens/worldselection/CreateWorldCallback;
private final onClose : Ljava/lang/Runnable;
private tempDataPackDir : Ljava/nio/file/Path;
private tempDataPackRepository : Lnet/minecraft/server/packs/repository/PackRepository;
private tabNavigationBar : Lnet/minecraft/client/gui/components/tabs/MenuTabBar;
public static openFresh(Lnet/minecraft/client/Minecraft;Ljava/lang/Runnable;)V
public static openFresh(Lnet/minecraft/client/Minecraft;Ljava/lang/Runnable;Lnet/minecraft/client/gui/screens/worldselection/CreateWorldCallback;)V
public static testWorld(Lnet/minecraft/client/Minecraft;Ljava/lang/Runnable;)V
private static openCreateWorldScreen(Lnet/minecraft/client/Minecraft;Ljava/lang/Runnable;Ljava/util/function/Function;Lnet/minecraft/client/gui/screens/worldselection/WorldCreationContextMapper;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/client/gui/screens/worldselection/CreateWorldCallback;)V
public static createFromExisting(Lnet/minecraft/client/Minecraft;Ljava/lang/Runnable;Lnet/minecraft/world/level/LevelSettings;Lnet/minecraft/client/gui/screens/worldselection/WorldCreationContext;Ljava/nio/file/Path;)Lnet/minecraft/client/gui/screens/worldselection/CreateWorldScreen;
private <init>(Lnet/minecraft/client/Minecraft;Ljava/lang/Runnable;Lnet/minecraft/client/gui/screens/worldselection/WorldCreationContext;Ljava/util/Optional;Ljava/util/OptionalLong;Lnet/minecraft/client/gui/screens/worldselection/CreateWorldCallback;)V
public getUiState()Lnet/minecraft/client/gui/screens/worldselection/WorldCreationUiState;
protected init()V
protected setInitialFocus()V
public repositionElements()V
private static queueLoadScreen(Lnet/minecraft/client/Minecraft;Lnet/minecraft/network/chat/Component;)V
private onCreate()V
private createWorldAndCleanup(Lnet/minecraft/core/LayeredRegistryAccess;Lnet/minecraft/world/level/storage/LevelDataAndDimensions$WorldDataAndGenSettings;Ljava/util/Optional;)V
private createNewWorld(Lnet/minecraft/core/LayeredRegistryAccess;Lnet/minecraft/world/level/storage/LevelDataAndDimensions$WorldDataAndGenSettings;Ljava/util/Optional;)Z
private createLevelSettings(Z)Lnet/minecraft/world/level/LevelSettings;
public keyPressed(Lnet/minecraft/client/input/KeyEvent;)Z
public onClose()V
public popScreen()V
public extractRenderState(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V
protected extractMenuBackground(Lnet/minecraft/client/gui/GuiGraphicsExtractor;)V
private getOrCreateTempDataPackDir()Ljava/nio/file/Path;
private openExperimentsScreen(Lnet/minecraft/world/level/WorldDataConfiguration;)V
private openDataPackSelectionScreen(Lnet/minecraft/world/level/WorldDataConfiguration;)V
private tryApplyNewDataPacks(Lnet/minecraft/server/packs/repository/PackRepository;ZLjava/util/function/Consumer;)V
private applyNewPackConfig(Lnet/minecraft/server/packs/repository/PackRepository;Lnet/minecraft/world/level/WorldDataConfiguration;Ljava/util/function/Consumer;)V
private static createDefaultLoadConfig(Lnet/minecraft/server/packs/repository/PackRepository;Lnet/minecraft/world/level/WorldDataConfiguration;)Lnet/minecraft/server/WorldLoader$InitConfig;
private removeTempDataPackDir()V
private static copyBetweenDirs(Ljava/nio/file/Path;Ljava/nio/file/Path;Ljava/nio/file/Path;)V
private static createNewWorldDirectory(Lnet/minecraft/client/Minecraft;Ljava/lang/String;Ljava/nio/file/Path;)Ljava/util/Optional;
public static createTempDataPackDirFromExistingWorld(Ljava/nio/file/Path;Lnet/minecraft/client/Minecraft;)Ljava/nio/file/Path;
private getDataPackSelectionSettings(Lnet/minecraft/world/level/WorldDataConfiguration;)Lcom/mojang/datafixers/util/Pair;
static synthetic access$000(Lnet/minecraft/client/gui/screens/worldselection/CreateWorldScreen;)Lnet/minecraft/client/gui/Font;
static synthetic access$100(Lnet/minecraft/client/gui/screens/worldselection/CreateWorldScreen;Lnet/minecraft/client/gui/components/events/GuiEventListener;)V
static synthetic access$200(Lnet/minecraft/client/gui/screens/worldselection/CreateWorldScreen;)Lnet/minecraft/client/gui/Font;
static synthetic access$300(Lnet/minecraft/client/gui/screens/worldselection/CreateWorldScreen;)Lnet/minecraft/client/gui/Font;
static synthetic access$400(Lnet/minecraft/client/gui/screens/worldselection/CreateWorldScreen;)Lnet/minecraft/client/gui/Font;
static synthetic access$500(Lnet/minecraft/client/gui/screens/worldselection/CreateWorldScreen;)Lnet/minecraft/client/Minecraft;
static synthetic access$600(Lnet/minecraft/client/gui/screens/worldselection/CreateWorldScreen;)Lnet/minecraft/client/Minecraft;
static synthetic access$700(Lnet/minecraft/client/gui/screens/worldselection/CreateWorldScreen;)Lnet/minecraft/client/Minecraft;
private static synthetic lambda$createTempDataPackDirFromExistingWorld$1(Lorg/apache/commons/lang3/mutable/MutableObject;Ljava/nio/file/Path;Ljava/nio/file/Path;)V
private static synthetic lambda$createTempDataPackDirFromExistingWorld$0(Ljava/nio/file/Path;Ljava/nio/file/Path;)Z
private static synthetic lambda$createNewWorldDirectory$1(Ljava/nio/file/Path;Ljava/nio/file/Path;Ljava/nio/file/Path;)V
private static synthetic lambda$createNewWorldDirectory$0(Ljava/nio/file/Path;Ljava/nio/file/Path;)Z
private static synthetic lambda$removeTempDataPackDir$0(Ljava/nio/file/Path;)V
private synthetic lambda$applyNewPackConfig$5(Ljava/util/function/Consumer;Ljava/lang/Void;Ljava/lang/Throwable;)Ljava/lang/Object;
private synthetic lambda$applyNewPackConfig$6(Ljava/util/function/Consumer;Z)V
private static synthetic lambda$applyNewPackConfig$4(Lnet/minecraft/client/gui/screens/worldselection/WorldCreationContext;)Lnet/minecraft/client/gui/screens/worldselection/WorldCreationContext;
private static synthetic lambda$applyNewPackConfig$3(Lnet/minecraft/server/packs/resources/CloseableResourceManager;Lnet/minecraft/server/ReloadableServerResources;Lnet/minecraft/core/LayeredRegistryAccess;Lnet/minecraft/client/gui/screens/worldselection/DataPackReloadCookie;)Lnet/minecraft/client/gui/screens/worldselection/WorldCreationContext;
private synthetic lambda$applyNewPackConfig$0(Lnet/minecraft/server/WorldLoader$DataLoadContext;)Lnet/minecraft/server/WorldLoader$DataLoadOutput;
private static synthetic lambda$applyNewPackConfig$2(Ljava/lang/String;)Ljava/lang/IllegalStateException;
private static synthetic lambda$applyNewPackConfig$1(Lcom/mojang/serialization/DynamicOps;Lcom/google/gson/JsonElement;)Lcom/mojang/serialization/DataResult;
private synthetic lambda$tryApplyNewDataPacks$1(Lnet/minecraft/server/packs/repository/PackRepository;Lnet/minecraft/world/level/WorldDataConfiguration;Ljava/util/function/Consumer;Z)V
private static synthetic lambda$tryApplyNewDataPacks$0(Ljava/util/List;Ljava/lang/String;)Z
private synthetic lambda$openDataPackSelectionScreen$0(Lnet/minecraft/server/packs/repository/PackRepository;)V
private synthetic lambda$openExperimentsScreen$0(Lnet/minecraft/server/packs/repository/PackRepository;)V
private synthetic lambda$onCreate$0(Lnet/minecraft/core/LayeredRegistryAccess;Lnet/minecraft/world/level/storage/LevelDataAndDimensions$WorldDataAndGenSettings;Lnet/minecraft/world/level/gamerules/GameRules;)V
private synthetic lambda$init$2(Lnet/minecraft/client/gui/components/AbstractWidget;)V
private synthetic lambda$init$1(Lnet/minecraft/client/gui/components/Button;)V
private synthetic lambda$init$0(Lnet/minecraft/client/gui/components/Button;)V
private static synthetic lambda$createFromExisting$0(Lnet/minecraft/client/gui/screens/worldselection/CreateWorldScreen;Lnet/minecraft/core/LayeredRegistryAccess;Lnet/minecraft/world/level/storage/LevelDataAndDimensions$WorldDataAndGenSettings;Ljava/util/Optional;Ljava/nio/file/Path;)Z
private static synthetic lambda$openCreateWorldScreen$1(Lnet/minecraft/client/gui/screens/worldselection/WorldCreationContextMapper;Lnet/minecraft/server/packs/resources/CloseableResourceManager;Lnet/minecraft/server/ReloadableServerResources;Lnet/minecraft/core/LayeredRegistryAccess;Lnet/minecraft/client/gui/screens/worldselection/DataPackReloadCookie;)Lnet/minecraft/client/gui/screens/worldselection/WorldCreationContext;
private static synthetic lambda$openCreateWorldScreen$0(Ljava/util/function/Function;Lnet/minecraft/server/WorldLoader$DataLoadContext;)Lnet/minecraft/server/WorldLoader$DataLoadOutput;
private static synthetic lambda$testWorld$2(Lnet/minecraft/client/gui/screens/worldselection/CreateWorldScreen;Lnet/minecraft/core/LayeredRegistryAccess;Lnet/minecraft/world/level/storage/LevelDataAndDimensions$WorldDataAndGenSettings;Ljava/util/Optional;Ljava/nio/file/Path;)Z
private static synthetic lambda$testWorld$1(Lnet/minecraft/server/WorldLoader$DataLoadContext;)Lnet/minecraft/world/level/levelgen/WorldGenSettings;
private static synthetic lambda$testWorld$0(Lnet/minecraft/server/ReloadableServerResources;Lnet/minecraft/core/LayeredRegistryAccess;Lnet/minecraft/client/gui/screens/worldselection/DataPackReloadCookie;)Lnet/minecraft/client/gui/screens/worldselection/WorldCreationContext;
private static synthetic lambda$openFresh$2(Lnet/minecraft/server/WorldLoader$DataLoadContext;)Lnet/minecraft/world/level/levelgen/WorldGenSettings;
private static synthetic lambda$openFresh$1(Lnet/minecraft/server/ReloadableServerResources;Lnet/minecraft/core/LayeredRegistryAccess;Lnet/minecraft/client/gui/screens/worldselection/DataPackReloadCookie;)Lnet/minecraft/client/gui/screens/worldselection/WorldCreationContext;
private static synthetic lambda$openFresh$0(Lnet/minecraft/client/gui/screens/worldselection/CreateWorldScreen;Lnet/minecraft/core/LayeredRegistryAccess;Lnet/minecraft/world/level/storage/LevelDataAndDimensions$WorldDataAndGenSettings;Ljava/util/Optional;Ljava/nio/file/Path;)Z
private static synthetic lambda$tabManager$1(Lnet/minecraft/client/gui/screens/worldselection/CreateWorldScreen;Lnet/minecraft/client/gui/components/AbstractWidget;)V
private static synthetic lambda$tabManager$0(Lnet/minecraft/client/gui/screens/worldselection/CreateWorldScreen;Lnet/minecraft/client/gui/components/AbstractWidget;)V
static <clinit>()V
```
