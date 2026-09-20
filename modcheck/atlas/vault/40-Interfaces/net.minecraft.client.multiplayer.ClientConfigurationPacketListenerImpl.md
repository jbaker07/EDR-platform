---
type: "interface"
fqcn: "net.minecraft.client.multiplayer.ClientConfigurationPacketListenerImpl"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.multiplayer.ClientConfigurationPacketListenerImpl

System: [[20-Systems/net.minecraft.client.multiplayer|net.minecraft.client.multiplayer]]

`class` public; extends `net/minecraft/client/multiplayer/ClientCommonPacketListenerImpl`; implements `net/minecraft/network/protocol/configuration/ClientConfigurationPacketListener`, `net/minecraft/network/TickablePacketListener`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getPacketContext` | `()Lnet/fabricmc/fabric/api/networking/v1/context/PacketContext;` | inherited_exact | invokevirtual@6 in `ClientConfigurationNetworking$Context.packetContext` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `<init>` | `(Lnet/minecraft/client/Minecraft;Lnet/minecraft/network/Connection;Lne` | name_only | @Inject at ['RETURN'] | client | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `handleConfigurationFinished` | `(Lnet/minecraft/network/protocol/configuration/ClientboundFinishConfig` | name_only | @Inject at ['INVOKE'] | client | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `handleConfigurationFinished` | `(Lnet/minecraft/network/protocol/configuration/ClientboundFinishConfig` | name_only | @Inject at ['NEW'] | client | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `handleSelectKnownPacks` | `(Lnet/minecraft/network/protocol/configuration/ClientboundSelectKnownP` | name_only | @Inject at ['TAIL'] | client | [[30-Mechanisms/fabric-recipe-api-v1|fabric-recipe-api-v1]] | direct_reference |

## Declared members (10 fields, 19 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
public static final DISCONNECTED_MESSAGE : Lnet/minecraft/network/chat/Component;
private final levelLoadTracker : Lnet/minecraft/client/multiplayer/LevelLoadTracker;
private final localGameProfile : Lcom/mojang/authlib/GameProfile;
private enabledFeatures : Lnet/minecraft/world/flag/FeatureFlagSet;
private final receivedRegistries : Lnet/minecraft/core/RegistryAccess$Frozen;
private final registryDataCollector : Lnet/minecraft/client/multiplayer/RegistryDataCollector;
private knownPacks : Lnet/minecraft/client/multiplayer/KnownPacksManager;
protected chatState : Lnet/minecraft/client/gui/components/ChatComponent$State;
private seenCodeOfConduct : Z
public <init>(Lnet/minecraft/client/Minecraft;Lnet/minecraft/network/Connection;Lnet/minecraft/client/multiplayer/CommonListenerCookie;)V
public isAcceptingMessages()Z
protected handleCustomPayload(Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload;)V
private handleUnknownCustomPayload(Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload;)V
public handleRegistryData(Lnet/minecraft/network/protocol/configuration/ClientboundRegistryDataPacket;)V
public handleUpdateTags(Lnet/minecraft/network/protocol/common/ClientboundUpdateTagsPacket;)V
public handleEnabledFeatures(Lnet/minecraft/network/protocol/configuration/ClientboundUpdateEnabledFeaturesPacket;)V
public handleSelectKnownPacks(Lnet/minecraft/network/protocol/configuration/ClientboundSelectKnownPacks;)V
public handleResetChat(Lnet/minecraft/network/protocol/configuration/ClientboundResetChatPacket;)V
private runWithResources(Ljava/util/function/Function;)Ljava/lang/Object;
public handleCodeOfConduct(Lnet/minecraft/network/protocol/configuration/ClientboundCodeOfConductPacket;)V
public handleConfigurationFinished(Lnet/minecraft/network/protocol/configuration/ClientboundFinishConfigurationPacket;)V
private static filterRegistries(Lnet/minecraft/core/RegistryAccess$Frozen;Ljava/util/stream/Stream;)Lnet/minecraft/core/RegistryAccess$Frozen;
public tick()V
public onDisconnect(Lnet/minecraft/network/DisconnectionDetails;)V
protected createDialogAccess()Lnet/minecraft/client/gui/screens/dialog/DialogConnectionAccess;
private synthetic lambda$handleConfigurationFinished$0(Lnet/minecraft/server/packs/resources/ResourceProvider;)Lnet/minecraft/core/RegistryAccess$Frozen;
private synthetic lambda$handleCodeOfConduct$0(Lnet/minecraft/client/gui/screens/Screen;Z)V
static <clinit>()V
```
