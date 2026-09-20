---
type: "interface"
fqcn: "net.minecraft.client.multiplayer.ClientConfigurationPacketListenerImpl"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.multiplayer.ClientConfigurationPacketListenerImpl

System: [[20-Systems/net.minecraft.client.multiplayer|net.minecraft.client.multiplayer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `<init>` | `@Inject at RETURN` | client | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `handleConfigurationFinished` | `@Inject at INVOKE Lnet/minecraft/network/Connection;setupInboundProtocol(Lnet/mi` | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `handleConfigurationFinished` | `@Inject at NEW (Lnet/minecraft/client/Minecraft;Lnet/minecraft/network/Connectio` | client | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `handleSelectKnownPacks` | `@Inject at TAIL` | client | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |

## Declared members (29, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.multiplayer.ClientConfigurationPacketListenerImpl extends net.minecraft.client.multiplayer.ClientCommonPacketListenerImpl implements net.minecraft.network.protocol.configuration.ClientConfigurationPacketListener,net.minecraft.network.TickablePacketListener {
    private static final org.slf4j.Logger LOGGER;
    public static final net.minecraft.network.chat.Component DISCONNECTED_MESSAGE;
    private final net.minecraft.client.multiplayer.LevelLoadTracker levelLoadTracker;
    private final com.mojang.authlib.GameProfile localGameProfile;
    private net.minecraft.world.flag.FeatureFlagSet enabledFeatures;
    private final net.minecraft.core.RegistryAccess$Frozen receivedRegistries;
    private final net.minecraft.client.multiplayer.RegistryDataCollector registryDataCollector;
    private net.minecraft.client.multiplayer.KnownPacksManager knownPacks;
    protected net.minecraft.client.gui.components.ChatComponent$State chatState;
    private boolean seenCodeOfConduct;
    public net.minecraft.client.multiplayer.ClientConfigurationPacketListenerImpl(net.minecraft.client.Minecraft, net.minecraft.network.Connection, net.minecraft.client.multiplayer.CommonListenerCookie);
    public boolean isAcceptingMessages();
    protected void handleCustomPayload(net.minecraft.network.protocol.common.custom.CustomPacketPayload);
    private void handleUnknownCustomPayload(net.minecraft.network.protocol.common.custom.CustomPacketPayload);
    public void handleRegistryData(net.minecraft.network.protocol.configuration.ClientboundRegistryDataPacket);
    public void handleUpdateTags(net.minecraft.network.protocol.common.ClientboundUpdateTagsPacket);
    public void handleEnabledFeatures(net.minecraft.network.protocol.configuration.ClientboundUpdateEnabledFeaturesPacket);
    public void handleSelectKnownPacks(net.minecraft.network.protocol.configuration.ClientboundSelectKnownPacks);
    public void handleResetChat(net.minecraft.network.protocol.configuration.ClientboundResetChatPacket);
    private <T> T runWithResources(java.util.function.Function<net.minecraft.server.packs.resources.ResourceProvider, T>);
    public void handleCodeOfConduct(net.minecraft.network.protocol.configuration.ClientboundCodeOfConductPacket);
    public void handleConfigurationFinished(net.minecraft.network.protocol.configuration.ClientboundFinishConfigurationPacket);
    private static net.minecraft.core.RegistryAccess$Frozen filterRegistries(net.minecraft.core.RegistryAccess$Frozen, java.util.stream.Stream<net.minecraft.resources.ResourceKey<? extends net.minecraft.core.Registry<?>>>);
    public void tick();
    public void onDisconnect(net.minecraft.network.DisconnectionDetails);
    protected net.minecraft.client.gui.screens.dialog.DialogConnectionAccess createDialogAccess();
    private net.minecraft.core.RegistryAccess$Frozen lambda$handleConfigurationFinished$0(net.minecraft.server.packs.resources.ResourceProvider);
    private void lambda$handleCodeOfConduct$0(net.minecraft.client.gui.screens.Screen, boolean);
    static {};
}
```
