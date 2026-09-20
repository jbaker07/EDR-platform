---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.worldselection.CreateWorldScreen"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.worldselection.CreateWorldScreen

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getUiState()Lnet/minecraft/client/gui/screens/worldselection/WorldCrea` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `openFresh(Lnet/minecraft/client/Minecraft;Ljava/lang/Runnable;)V` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `getDataPackSelectionSettings` | `@Inject at INVOKE Lnet/minecraft/server/packs/repository/PackRepository;reload()` | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| injects_into | `onCreate` | `@Inject at INVOKE Lnet/minecraft/client/gui/screens/worldselection/WorldOpenFlow` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| injects_into | `openCreateWorldScreen(Lnet/minecraft/client/Minecraft;Ljava/lang/Runnable;Ljava/util/function/Function;Lnet/minecraft/client/gui/screens/worldselection/WorldCreationContextMapper;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/client/gui/screens/worldselection/CreateWorldCallback;)V` | `@ModifyVariable at INVOKE Lnet/minecraft/client/gui/screens/worldselection/Creat` | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| wraps | `openCreateWorldScreen(Lnet/minecraft/client/Minecraft;Ljava/lang/Runnable;Ljava/util/function/Function;Lnet/minecraft/client/gui/screens/worldselection/WorldCreationContextMapper;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/client/gui/screens/worldselection/CreateWorldCallback;)V` | `@Redirect at FIELD Lnet/minecraft/world/level/WorldDataConfiguration;DEFAULT:Lne` | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (93, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.gui.screens.worldselection.CreateWorldScreen extends net.minecraft.client.gui.screens.Screen {
    private static final int GROUP_BOTTOM;
    private static final int TAB_COLUMN_WIDTH;
    private static final org.slf4j.Logger LOGGER;
    private static final java.lang.String TEMP_WORLD_PREFIX;
    private static final net.minecraft.network.chat.Component GAME_MODEL_LABEL;
    private static final net.minecraft.network.chat.Component NAME_LABEL;
    private static final net.minecraft.network.chat.Component EXPERIMENTS_LABEL;
    private static final net.minecraft.network.chat.Component ALLOW_COMMANDS_INFO;
    private static final net.minecraft.network.chat.Component PREPARING_WORLD_DATA;
    private static final int HORIZONTAL_BUTTON_SPACING;
    private static final int VERTICAL_BUTTON_SPACING;
    public static final net.minecraft.resources.Identifier TAB_HEADER_BACKGROUND;
    private final net.minecraft.client.gui.layouts.HeaderAndFooterLayout layout;
    private final net.minecraft.client.gui.screens.worldselection.WorldCreationUiState uiState;
    private final net.minecraft.client.gui.components.tabs.TabManager tabManager;
    private boolean recreated;
    private final net.minecraft.world.level.validation.DirectoryValidator packValidator;
    private final net.minecraft.client.gui.screens.worldselection.CreateWorldCallback createWorldCallback;
    private final java.lang.Runnable onClose;
    private java.nio.file.Path tempDataPackDir;
    private net.minecraft.server.packs.repository.PackRepository tempDataPackRepository;
    private net.minecraft.client.gui.components.tabs.MenuTabBar tabNavigationBar;
    public static void openFresh(net.minecraft.client.Minecraft, java.lang.Runnable);
    public static void openFresh(net.minecraft.client.Minecraft, java.lang.Runnable, net.minecraft.client.gui.screens.worldselection.CreateWorldCallback);
    public static void testWorld(net.minecraft.client.Minecraft, java.lang.Runnable);
    private static void openCreateWorldScreen(net.minecraft.client.Minecraft, java.lang.Runnable, java.util.function.Function<net.minecraft.server.WorldLoader$DataLoadContext, net.minecraft.world.level.levelgen.WorldGenSettings>, net.minecraft.client.gui.screens.worldselection.WorldCreationContextMapper, net.minecraft.resources.ResourceKey<net.minecraft.world.level.levelgen.presets.WorldPreset>, net.minecraft.client.gui.screens.worldselection.CreateWorldCallback);
    public static net.minecraft.client.gui.screens.worldselection.CreateWorldScreen createFromExisting(net.minecraft.client.Minecraft, java.lang.Runnable, net.minecraft.world.level.LevelSettings, net.minecraft.client.gui.screens.worldselection.WorldCreationContext, java.nio.file.Path);
    private net.minecraft.client.gui.screens.worldselection.CreateWorldScreen(net.minecraft.client.Minecraft, java.lang.Runnable, net.minecraft.client.gui.screens.worldselection.WorldCreationContext, java.util.Optional<net.minecraft.resources.ResourceKey<net.minecraft.world.level.levelgen.presets.WorldPreset>>, java.util.OptionalLong, net.minecraft.client.gui.screens.worldselection.CreateWorldCallback);
    public net.minecraft.client.gui.screens.worldselection.WorldCreationUiState getUiState();
    protected void init();
    protected void setInitialFocus();
    public void repositionElements();
    private static void queueLoadScreen(net.minecraft.client.Minecraft, net.minecraft.network.chat.Component);
    private void onCreate();
    private void createWorldAndCleanup(net.minecraft.core.LayeredRegistryAccess<net.minecraft.server.RegistryLayer>, net.minecraft.world.level.storage.LevelDataAndDimensions$WorldDataAndGenSettings, java.util.Optional<net.minecraft.world.level.gamerules.GameRules>);
    private boolean createNewWorld(net.minecraft.core.LayeredRegistryAccess<net.minecraft.server.RegistryLayer>, net.minecraft.world.level.storage.LevelDataAndDimensions$WorldDataAndGenSettings, java.util.Optional<net.minecraft.world.level.gamerules.GameRules>);
    private net.minecraft.world.level.LevelSettings createLevelSettings(boolean);
    public boolean keyPressed(net.minecraft.client.input.KeyEvent);
    public void onClose();
    public void popScreen();
    public void extractRenderState(net.minecraft.client.gui.GuiGraphicsExtractor, int, int, float);
    protected void extractMenuBackground(net.minecraft.client.gui.GuiGraphicsExtractor);
    private java.nio.file.Path getOrCreateTempDataPackDir();
    private void openExperimentsScreen(net.minecraft.world.level.WorldDataConfiguration);
    private void openDataPackSelectionScreen(net.minecraft.world.level.WorldDataConfiguration);
    private void tryApplyNewDataPacks(net.minecraft.server.packs.repository.PackRepository, boolean, java.util.function.Consumer<net.minecraft.world.level.WorldDataConfiguration>);
    private void applyNewPackConfig(net.minecraft.server.packs.repository.PackRepository, net.minecraft.world.level.WorldDataConfiguration, java.util.function.Consumer<net.minecraft.world.level.WorldDataConfiguration>);
    private static net.minecraft.server.WorldLoader$InitConfig createDefaultLoadConfig(net.minecraft.server.packs.repository.PackRepository, net.minecraft.world.level.WorldDataConfiguration);
    private void removeTempDataPackDir();
    private static void copyBetweenDirs(java.nio.file.Path, java.nio.file.Path, java.nio.file.Path);
    private static java.util.Optional<net.minecraft.world.level.storage.LevelStorageSource$LevelStorageAccess> createNewWorldDirectory(net.minecraft.client.Minecraft, java.lang.String, java.nio.file.Path);
    public static java.nio.file.Path createTempDataPackDirFromExistingWorld(java.nio.file.Path, net.minecraft.client.Minecraft);
    private com.mojang.datafixers.util.Pair<java.nio.file.Path, net.minecraft.server.packs.repository.PackRepository> getDataPackSelectionSettings(net.minecraft.world.level.WorldDataConfiguration);
    static net.minecraft.client.gui.Font access$000(net.minecraft.client.gui.screens.worldselection.CreateWorldScreen);
    static void access$100(net.minecraft.client.gui.screens.worldselection.CreateWorldScreen, net.minecraft.client.gui.components.events.GuiEventListener);
    static net.minecraft.client.gui.Font access$200(net.minecraft.client.gui.screens.worldselection.CreateWorldScreen);
    static net.minecraft.client.gui.Font access$300(net.minecraft.client.gui.screens.worldselection.CreateWorldScreen);
    static net.minecraft.client.gui.Font access$400(net.minecraft.client.gui.screens.worldselection.CreateWorldScreen);
    static net.minecraft.client.Minecraft access$500(net.minecraft.client.gui.screens.worldselection.CreateWorldScreen);
    static net.minecraft.client.Minecraft access$600(net.minecraft.client.gui.screens.worldselection.CreateWorldScreen);
    static net.minecraft.client.Minecraft access$700(net.minecraft.client.gui.screens.worldselection.CreateWorldScreen);
    private static void lambda$createTempDataPackDirFromExistingWorld$1(org.apache.commons.lang3.mutable.MutableObject, java.nio.file.Path, java.nio.file.Path);
    private static boolean lambda$createTempDataPackDirFromExistingWorld$0(java.nio.file.Path, java.nio.file.Path);
    private static void lambda$createNewWorldDirectory$1(java.nio.file.Path, java.nio.file.Path, java.nio.file.Path);
    private static boolean lambda$createNewWorldDirectory$0(java.nio.file.Path, java.nio.file.Path);
    private static void lambda$removeTempDataPackDir$0(java.nio.file.Path);
    private java.lang.Object lambda$applyNewPackConfig$5(java.util.function.Consumer, java.lang.Void, java.lang.Throwable);
    private void lambda$applyNewPackConfig$6(java.util.function.Consumer, boolean);
    private static net.minecraft.client.gui.screens.worldselection.WorldCreationContext lambda$applyNewPackConfig$4(net.minecraft.client.gui.screens.worldselection.WorldCreationContext);
    private static net.minecraft.client.gui.screens.worldselection.WorldCreationContext lambda$applyNewPackConfig$3(net.minecraft.server.packs.resources.CloseableResourceManager, net.minecraft.server.ReloadableServerResources, net.minecraft.core.LayeredRegistryAccess, net.minecraft.client.gui.screens.worldselection.DataPackReloadCookie);
    private net.minecraft.server.WorldLoader$DataLoadOutput lambda$applyNewPackConfig$0(net.minecraft.server.WorldLoader$DataLoadContext);
    private static java.lang.IllegalStateException lambda$applyNewPackConfig$2(java.lang.String);
    private static com.mojang.serialization.DataResult lambda$applyNewPackConfig$1(com.mojang.serialization.DynamicOps, com.google.gson.JsonElement);
    private void lambda$tryApplyNewDataPacks$1(net.minecraft.server.packs.repository.PackRepository, net.minecraft.world.level.WorldDataConfiguration, java.util.function.Consumer, boolean);
    private static boolean lambda$tryApplyNewDataPacks$0(java.util.List, java.lang.String);
    private void lambda$openDataPackSelectionScreen$0(net.minecraft.server.packs.repository.PackRepository);
    private void lambda$openExperimentsScreen$0(net.minecraft.server.packs.repository.PackRepository);
    private void lambda$onCreate$0(net.minecraft.core.LayeredRegistryAccess, net.minecraft.world.level.storage.LevelDataAndDimensions$WorldDataAndGenSettings, net.minecraft.world.level.gamerules.GameRules);
    private void lambda$init$2(net.minecraft.client.gui.components.AbstractWidget);
    private void lambda$init$1(net.minecraft.client.gui.components.Button);
    private void lambda$init$0(net.minecraft.client.gui.components.Button);
    private static boolean lambda$createFromExisting$0(net.minecraft.client.gui.screens.worldselection.CreateWorldScreen, net.minecraft.core.LayeredRegistryAccess, net.minecraft.world.level.storage.LevelDataAndDimensions$WorldDataAndGenSettings, java.util.Optional, java.nio.file.Path);
    private static net.minecraft.client.gui.screens.worldselection.WorldCreationContext lambda$openCreateWorldScreen$1(net.minecraft.client.gui.screens.worldselection.WorldCreationContextMapper, net.minecraft.server.packs.resources.CloseableResourceManager, net.minecraft.server.ReloadableServerResources, net.minecraft.core.LayeredRegistryAccess, net.minecraft.client.gui.screens.worldselection.DataPackReloadCookie);
    private static net.minecraft.server.WorldLoader$DataLoadOutput lambda$openCreateWorldScreen$0(java.util.function.Function, net.minecraft.server.WorldLoader$DataLoadContext);
    private static boolean lambda$testWorld$2(net.minecraft.client.gui.screens.worldselection.CreateWorldScreen, net.minecraft.core.LayeredRegistryAccess, net.minecraft.world.level.storage.LevelDataAndDimensions$WorldDataAndGenSettings, java.util.Optional, java.nio.file.Path);
    private static net.minecraft.world.level.levelgen.WorldGenSettings lambda$testWorld$1(net.minecraft.server.WorldLoader$DataLoadContext);
    private static net.minecraft.client.gui.screens.worldselection.WorldCreationContext lambda$testWorld$0(net.minecraft.server.ReloadableServerResources, net.minecraft.core.LayeredRegistryAccess, net.minecraft.client.gui.screens.worldselection.DataPackReloadCookie);
    private static net.minecraft.world.level.levelgen.WorldGenSettings lambda$openFresh$2(net.minecraft.server.WorldLoader$DataLoadContext);
    private static net.minecraft.client.gui.screens.worldselection.WorldCreationContext lambda$openFresh$1(net.minecraft.server.ReloadableServerResources, net.minecraft.core.LayeredRegistryAccess, net.minecraft.client.gui.screens.worldselection.DataPackReloadCookie);
    private static boolean lambda$openFresh$0(net.minecraft.client.gui.screens.worldselection.CreateWorldScreen, net.minecraft.core.LayeredRegistryAccess, net.minecraft.world.level.storage.LevelDataAndDimensions$WorldDataAndGenSettings, java.util.Optional, java.nio.file.Path);
    private static void lambda$tabManager$1(net.minecraft.client.gui.screens.worldselection.CreateWorldScreen, net.minecraft.client.gui.components.AbstractWidget);
    private static void lambda$tabManager$0(net.minecraft.client.gui.screens.worldselection.CreateWorldScreen, net.minecraft.client.gui.components.AbstractWidget);
    static {};
}
```
