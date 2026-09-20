---
type: "interface"
fqcn: "net.minecraft.server.network.ServerConfigurationPacketListenerImpl"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.network.ServerConfigurationPacketListenerImpl

System: [[20-Systems/net.minecraft.server.network|net.minecraft.server.network]]

`class` public; extends `net/minecraft/server/network/ServerCommonPacketListenerImpl`; implements `net/minecraft/network/protocol/configuration/ServerConfigurationPacketListener`, `net/minecraft/network/TickablePacketListener`, `net/fabricmc/fabric/api/networking/v1/FabricServerConfigurationPacketListenerImpl`; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `addTask` | `(Lnet/minecraft/server/network/ConfigurationTask;)V` | inherited_exact | invokevirtual@18 in `AttachmentSync.lambda$onInitialize$0` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `addTask` | `(Lnet/minecraft/server/network/ConfigurationTask;)V` | inherited_exact | invokevirtual@24 in `CommonPacketsImpl.lambda$init$2` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `addTask` | `(Lnet/minecraft/server/network/ConfigurationTask;)V` | inherited_exact | invokevirtual@46 in `CommonPacketsImpl.lambda$init$2` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `addTask` | `(Lnet/minecraft/server/network/ConfigurationTask;)V` | inherited_exact | invokevirtual@18 in `CustomIngredientSync.lambda$onInitialize$0` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `addTask` | `(Lnet/minecraft/server/network/ConfigurationTask;)V` | inherited_exact | invokevirtual@80 in `RegistrySyncManager.configureClient` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `completeTask` | `(Lnet/minecraft/server/network/ConfigurationTask$Type;)V` | inherited_exact | invokevirtual@32 in `AttachmentSync.lambda$onInitialize$1` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `completeTask` | `(Lnet/minecraft/server/network/ConfigurationTask$Type;)V` | inherited_exact | invokevirtual@119 in `CommonPacketsImpl.lambda$init$1` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `completeTask` | `(Lnet/minecraft/server/network/ConfigurationTask$Type;)V` | inherited_exact | invokevirtual@27 in `CommonPacketsImpl.lambda$init$0` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `completeTask` | `(Lnet/minecraft/server/network/ConfigurationTask$Type;)V` | inherited_exact | invokevirtual@32 in `CustomIngredientSync.lambda$onInitialize$1` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `completeTask` | `(Lnet/minecraft/server/network/ConfigurationTask$Type;)V` | inherited_exact | invokevirtual@9 in `FabricRegistryInit.lambda$onInitialize$0` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `disconnect` | `(Lnet/minecraft/network/chat/Component;)V` | inherited_exact | invokevirtual@66 in `RegistrySyncManager.configureClient` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `finishCurrentTask` | `(Lnet/minecraft/server/network/ConfigurationTask$Type;)V` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | declared |
| calls | `getOwner` | `()Lcom/mojang/authlib/GameProfile;` | inherited_exact | invokevirtual@14 in `ServerConfigurationNetworkAddon.<init>` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getOwner` | `()Lcom/mojang/authlib/GameProfile;` | inherited_exact | invokevirtual@12 in `RegistrySyncManager.configureClient` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `getPacketContext` | `()Lnet/fabricmc/fabric/api/networking/v1/context/PacketContext;` | inherited_exact | invokevirtual@11 in `AttachmentSync.lambda$onInitialize$1` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `getPacketContext` | `()Lnet/fabricmc/fabric/api/networking/v1/context/PacketContext;` | inherited_exact | invokevirtual@6 in `ServerConfigurationNetworking$Context.packetContext` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `getPacketContext` | `()Lnet/fabricmc/fabric/api/networking/v1/context/PacketContext;` | inherited_exact | invokevirtual@1 in `ExtendedBlockParticleOptionSync.lambda$onInitialize$0` | unknown | [[30-Mechanisms/fabric-particles-v1|fabric-particles-v1]] | direct_reference |
| calls | `getPacketContext` | `()Lnet/fabricmc/fabric/api/networking/v1/context/PacketContext;` | inherited_exact | invokevirtual@11 in `CustomIngredientSync.lambda$onInitialize$1` | unknown | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |
| calls | `isAcceptingMessages` | `()Z` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | declared |
| calls | `send` | `(Lnet/minecraft/network/protocol/Packet;)V` | inherited_exact | invokevirtual@43 in `ServerConfigurationNetworking.send` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `send` | `(Lnet/minecraft/network/protocol/Packet;Lio/netty/channel/ChannelFutur` | inherited_exact | invokevirtual@6 in `ServerConfigurationNetworkAddon.sendPacket` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `startConfiguration` | `()V` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | declared |
| calls | `startConfiguration` | `()V` | exact | invokevirtual@31 in `ServerConfigurationNetworkAddon.receiveRegistration` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `startConfiguration` | `()V` | exact | invokevirtual@21 in `ServerConfigurationNetworkAddon.onPong` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `startConfiguration` | `()V` | exact | invokevirtual@1 in `DebugConfigCommandMixin.sendConfigurations` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `startConfiguration` | `()V` | exact | invokevirtual@50 in `ServerGamePacketListenerImplMixin.onAcknowledgeReconfiguration` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `<init>` | `(Lnet/minecraft/server/MinecraftServer;Lnet/minecraft/network/Connecti` | name_only | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `startConfiguration` | `()V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `startConfiguration` | `()V` | name_only | @ModifyArg at ['INVOKE'] | both | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |
| reads | `configurationTasks` | `Ljava/util/Queue;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | declared |
| reads | `currentTask` | `Lnet/minecraft/server/network/ConfigurationTask;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | declared |
| wraps | `handleConfigurationFinished` | `(Lnet/minecraft/network/protocol/configuration/ServerboundFinishConfig` | name_only | @WrapOperation at ['INVOKE'] | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (9 fields, 19 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final DISCONNECT_REASON_INVALID_DATA : Lnet/minecraft/network/chat/Component;
private static final DISCONNECT_REASON_CONFIGURATION_ERROR : Lnet/minecraft/network/chat/Component;
private final gameProfile : Lcom/mojang/authlib/GameProfile;
private final configurationTasks : Ljava/util/Queue;
private currentTask : Lnet/minecraft/server/network/ConfigurationTask;
private clientInformation : Lnet/minecraft/server/level/ClientInformation;
private synchronizeRegistriesTask : Lnet/minecraft/server/network/config/SynchronizeRegistriesTask;
private prepareSpawnTask : Lnet/minecraft/server/network/config/PrepareSpawnTask;
public <init>(Lnet/minecraft/server/MinecraftServer;Lnet/minecraft/network/Connection;Lnet/minecraft/server/network/CommonListenerCookie;)V
protected playerProfile()Lcom/mojang/authlib/GameProfile;
public onDisconnect(Lnet/minecraft/network/DisconnectionDetails;)V
public isAcceptingMessages()Z
public startConfiguration()V
public returnToWorld()V
private addOptionalTasks()V
public handleClientInformation(Lnet/minecraft/network/protocol/common/ServerboundClientInformationPacket;)V
public handleResourcePackResponse(Lnet/minecraft/network/protocol/common/ServerboundResourcePackPacket;)V
public handleSelectKnownPacks(Lnet/minecraft/network/protocol/configuration/ServerboundSelectKnownPacks;)V
public handleAcceptCodeOfConduct(Lnet/minecraft/network/protocol/configuration/ServerboundAcceptCodeOfConductPacket;)V
public handleConfigurationFinished(Lnet/minecraft/network/protocol/configuration/ServerboundFinishConfigurationPacket;)V
public tick()V
private startNextTask()V
private finishCurrentTask(Lnet/minecraft/server/network/ConfigurationTask$Type;)V
private synthetic lambda$addOptionalTasks$1(Lnet/minecraft/server/MinecraftServer$ServerResourcePackInfo;)V
private synthetic lambda$addOptionalTasks$0(Ljava/util/Map;)Ljava/lang/String;
private static synthetic lambda$startConfiguration$0(Lnet/minecraft/server/packs/PackResources;)Ljava/util/stream/Stream;
static <clinit>()V
```
