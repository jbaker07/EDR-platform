---
type: "interface"
fqcn: "net.minecraft.server.network.ServerConfigurationPacketListenerImpl"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.network.ServerConfigurationPacketListenerImpl

System: [[20-Systems/net.minecraft.server.network|net.minecraft.server.network]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `addTask(Lnet/minecraft/server/network/ConfigurationTask;)V` | `` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `addTask(Lnet/minecraft/server/network/ConfigurationTask;)V` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `addTask(Lnet/minecraft/server/network/ConfigurationTask;)V` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `addTask(Lnet/minecraft/server/network/ConfigurationTask;)V` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `completeTask(Lnet/minecraft/server/network/ConfigurationTask$Type;)V` | `` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `completeTask(Lnet/minecraft/server/network/ConfigurationTask$Type;)V` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `completeTask(Lnet/minecraft/server/network/ConfigurationTask$Type;)V` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `completeTask(Lnet/minecraft/server/network/ConfigurationTask$Type;)V` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `disconnect(Lnet/minecraft/network/chat/Component;)V` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getOwner()Lcom/mojang/authlib/GameProfile;` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getOwner()Lcom/mojang/authlib/GameProfile;` | `` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getPacketContext()Lnet/fabricmc/fabric/api/networking/v1/context/PacketConte` | `` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `getPacketContext()Lnet/fabricmc/fabric/api/networking/v1/context/PacketConte` | `` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| calls | `getPacketContext()Lnet/fabricmc/fabric/api/networking/v1/context/PacketConte` | `` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `send(Lnet/minecraft/network/protocol/Packet;Lio/netty/channel/Ch` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `startConfiguration()V` | `` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `startConfiguration()V` | `` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `startConfiguration()V` | `` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `<init>` | `@Inject at RETURN` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `startConfiguration` | `@Inject at HEAD` | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `startConfiguration` | `@ModifyArg at INVOKE Lnet/minecraft/server/network/config/SynchronizeRegistriesT` | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (28, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.server.network.ServerConfigurationPacketListenerImpl extends net.minecraft.server.network.ServerCommonPacketListenerImpl implements net.minecraft.network.protocol.configuration.ServerConfigurationPacketListener,net.minecraft.network.TickablePacketListener {
    private static final org.slf4j.Logger LOGGER;
    private static final net.minecraft.network.chat.Component DISCONNECT_REASON_INVALID_DATA;
    private static final net.minecraft.network.chat.Component DISCONNECT_REASON_CONFIGURATION_ERROR;
    private final com.mojang.authlib.GameProfile gameProfile;
    private final java.util.Queue<net.minecraft.server.network.ConfigurationTask> configurationTasks;
    private net.minecraft.server.network.ConfigurationTask currentTask;
    private net.minecraft.server.level.ClientInformation clientInformation;
    private net.minecraft.server.network.config.SynchronizeRegistriesTask synchronizeRegistriesTask;
    private net.minecraft.server.network.config.PrepareSpawnTask prepareSpawnTask;
    public net.minecraft.server.network.ServerConfigurationPacketListenerImpl(net.minecraft.server.MinecraftServer, net.minecraft.network.Connection, net.minecraft.server.network.CommonListenerCookie);
    protected com.mojang.authlib.GameProfile playerProfile();
    public void onDisconnect(net.minecraft.network.DisconnectionDetails);
    public boolean isAcceptingMessages();
    public void startConfiguration();
    public void returnToWorld();
    private void addOptionalTasks();
    public void handleClientInformation(net.minecraft.network.protocol.common.ServerboundClientInformationPacket);
    public void handleResourcePackResponse(net.minecraft.network.protocol.common.ServerboundResourcePackPacket);
    public void handleSelectKnownPacks(net.minecraft.network.protocol.configuration.ServerboundSelectKnownPacks);
    public void handleAcceptCodeOfConduct(net.minecraft.network.protocol.configuration.ServerboundAcceptCodeOfConductPacket);
    public void handleConfigurationFinished(net.minecraft.network.protocol.configuration.ServerboundFinishConfigurationPacket);
    public void tick();
    private void startNextTask();
    private void finishCurrentTask(net.minecraft.server.network.ConfigurationTask$Type);
    private void lambda$addOptionalTasks$1(net.minecraft.server.MinecraftServer$ServerResourcePackInfo);
    private java.lang.String lambda$addOptionalTasks$0(java.util.Map);
    private static java.util.stream.Stream lambda$startConfiguration$0(net.minecraft.server.packs.PackResources);
    static {};
}
```
