---
type: "mechanism"
module: "fabric-networking-api-v1"
version: "6.3.8+fcdff87f5d"
sha256: "dfff56a878bba654646e986d90cf05913d7a914ad6c1292874de1ad505474544"
lifecycle: "stable"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# fabric-networking-api-v1

**Version** `6.3.8+fcdff87f5d` -- **artifact sha256** `dfff56a878bba654646e986d90cf05913d7a914ad6c1292874de1ad505474544`

## Declared (fabric.mod.json)

- environment: `*`
- depends: `{"fabricloader": ">=0.19.3", "fabric-api-base": "*"}`
- entrypoints: `{"main": ["net.fabricmc.fabric.impl.networking.CommonPacketsImpl::init", "net.fabricmc.fabric.impl.networking.NetworkingImpl::init"], "client": ["net.fabricmc.fabric.impl.networking.client.ClientNetworkingImpl::clientInit"]}`
- mixin configs: `["fabric-networking-api-v1.mixins.json", {"config": "fabric-networking-api-v1.client.mixins.json", "environment": "client"}]`
- access widener: `fabric-networking-api-v1.classtweaker`
- mixin classes: 37 found by annotation, 37 declared in configs; extraction failures: 0

## Events this module publishes

- [[50-Interactions/events/net.fabricmc.fabric.api.client.networking.v1.ClientConfigurationConnectionEvents.COMPLETE|ClientConfigurationConnectionEvents.COMPLETE]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.networking.v1.ClientConfigurationConnectionEvents.DISCONNECT|ClientConfigurationConnectionEvents.DISCONNECT]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.networking.v1.ClientConfigurationConnectionEvents.INIT|ClientConfigurationConnectionEvents.INIT]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.networking.v1.ClientConfigurationConnectionEvents.READY|ClientConfigurationConnectionEvents.READY]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.networking.v1.ClientConfigurationConnectionEvents.START|ClientConfigurationConnectionEvents.START]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.networking.v1.ClientLoginConnectionEvents.DISCONNECT|ClientLoginConnectionEvents.DISCONNECT]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.networking.v1.ClientLoginConnectionEvents.INIT|ClientLoginConnectionEvents.INIT]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.networking.v1.ClientLoginConnectionEvents.QUERY_START|ClientLoginConnectionEvents.QUERY_START]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.networking.v1.ClientPlayConnectionEvents.DISCONNECT|ClientPlayConnectionEvents.DISCONNECT]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.networking.v1.ClientPlayConnectionEvents.INIT|ClientPlayConnectionEvents.INIT]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.networking.v1.ClientPlayConnectionEvents.JOIN|ClientPlayConnectionEvents.JOIN]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.networking.v1.ServerboundConfigurationChannelEvents.REGISTER|ServerboundConfigurationChannelEvents.REGISTER]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.networking.v1.ServerboundConfigurationChannelEvents.UNREGISTER|ServerboundConfigurationChannelEvents.UNREGISTER]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.networking.v1.ServerboundPlayChannelEvents.REGISTER|ServerboundPlayChannelEvents.REGISTER]]
- [[50-Interactions/events/net.fabricmc.fabric.api.client.networking.v1.ServerboundPlayChannelEvents.UNREGISTER|ServerboundPlayChannelEvents.UNREGISTER]]
- [[50-Interactions/events/net.fabricmc.fabric.api.networking.v1.ClientboundConfigurationChannelEvents.REGISTER|ClientboundConfigurationChannelEvents.REGISTER]]
- [[50-Interactions/events/net.fabricmc.fabric.api.networking.v1.ClientboundConfigurationChannelEvents.UNREGISTER|ClientboundConfigurationChannelEvents.UNREGISTER]]
- [[50-Interactions/events/net.fabricmc.fabric.api.networking.v1.ClientboundPlayChannelEvents.REGISTER|ClientboundPlayChannelEvents.REGISTER]]
- [[50-Interactions/events/net.fabricmc.fabric.api.networking.v1.ClientboundPlayChannelEvents.UNREGISTER|ClientboundPlayChannelEvents.UNREGISTER]]
- [[50-Interactions/events/net.fabricmc.fabric.api.networking.v1.EntityTrackingEvents.START_TRACKING|EntityTrackingEvents.START_TRACKING]]
- [[50-Interactions/events/net.fabricmc.fabric.api.networking.v1.EntityTrackingEvents.STOP_TRACKING|EntityTrackingEvents.STOP_TRACKING]]
- [[50-Interactions/events/net.fabricmc.fabric.api.networking.v1.ServerConfigurationConnectionEvents.BEFORE_CONFIGURE|ServerConfigurationConnectionEvents.BEFORE_CONFIGURE]]
- [[50-Interactions/events/net.fabricmc.fabric.api.networking.v1.ServerConfigurationConnectionEvents.CONFIGURE|ServerConfigurationConnectionEvents.CONFIGURE]]
- [[50-Interactions/events/net.fabricmc.fabric.api.networking.v1.ServerConfigurationConnectionEvents.DISCONNECT|ServerConfigurationConnectionEvents.DISCONNECT]]
- [[50-Interactions/events/net.fabricmc.fabric.api.networking.v1.ServerLoginConnectionEvents.DISCONNECT|ServerLoginConnectionEvents.DISCONNECT]]
- [[50-Interactions/events/net.fabricmc.fabric.api.networking.v1.ServerLoginConnectionEvents.INIT|ServerLoginConnectionEvents.INIT]]
- [[50-Interactions/events/net.fabricmc.fabric.api.networking.v1.ServerLoginConnectionEvents.QUERY_START|ServerLoginConnectionEvents.QUERY_START]]
- [[50-Interactions/events/net.fabricmc.fabric.api.networking.v1.ServerPlayConnectionEvents.DISCONNECT|ServerPlayConnectionEvents.DISCONNECT]]
- [[50-Interactions/events/net.fabricmc.fabric.api.networking.v1.ServerPlayConnectionEvents.INIT|ServerPlayConnectionEvents.INIT]]
- [[50-Interactions/events/net.fabricmc.fabric.api.networking.v1.ServerPlayConnectionEvents.JOIN|ServerPlayConnectionEvents.JOIN]]

## Vanilla methods this module modifies

One row per (injection, selector). `resolution` says how the selector matched the processed jar; `points` are the @At targets with their own resolution.

| vanilla method | descriptor | resolution | injector | points | env | priority | handler |
|---|---|---|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientCommonPacketListenerImpl|ClientCommonPacketListenerImpl]].`handleCustomPayload` | `(Lnet/minecraft/network/protocol/common/ClientboundCustomPayloadPacket;)V` | exact | @Inject | HEAD | client | 1000 (default) | `ClientCommonPacketListenerImplMixin.onCustomPayload` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientConfigurationPacketListenerImpl|ClientConfigurationPacketListenerImpl]].`<init>` | `(Lnet/minecraft/client/Minecraft;Lnet/minecraft/network/Connection;Lnet/minecraft/client/multiplayer/CommonListenerCookie;)V` | name_only | @Inject | RETURN | client | 999 | `ClientConfigurationPacketListenerImplMixin.initAddon` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientConfigurationPacketListenerImpl|ClientConfigurationPacketListenerImpl]].`handleConfigurationFinished` | `(Lnet/minecraft/network/protocol/configuration/ClientboundFinishConfigurationPacket;)V` | name_only | @Inject | NEW `(Lnet/minecraft/client/Minecraft;Lnet/minecraft/network/Connection;Lnet/minecraft/client/multiplayer/CommonListenerCookie;)Lnet/minecraft/client/multiplayer/ClientPacketListener;` (exact) | client | 999 | `ClientConfigurationPacketListenerImplMixin.handleComplete` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientHandshakePacketListenerImpl|ClientHandshakePacketListenerImpl]].`<init>` | `(Lnet/minecraft/network/Connection;Lnet/minecraft/client/Minecraft;Lnet/minecraft/client/multiplayer/ServerData;Lnet/minecraft/client/gui/screens/Screen;ZLjava/time/Duration;Ljava/util/function/Consumer;Lnet/minecraft/client/multiplayer/LevelLoadTracker;Lnet/minecraft/client/multiplayer/TransferState;)V` | name_only | @Inject | RETURN | client | 1000 (default) | `ClientHandshakePacketListenerImplMixin.initAddon` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientHandshakePacketListenerImpl|ClientHandshakePacketListenerImpl]].`handleCustomQuery` | `(Lnet/minecraft/network/protocol/login/ClientboundCustomQueryPacket;)V` | name_only | @Inject | INVOKE `Ljava/util/function/Consumer;accept(Ljava/lang/Object;)V` (exact) | client | 1000 (default) | `ClientHandshakePacketListenerImplMixin.handleQueryRequest` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientHandshakePacketListenerImpl|ClientHandshakePacketListenerImpl]].`handleLoginFinished` | `(Lnet/minecraft/network/protocol/login/ClientboundLoginFinishedPacket;)V` | name_only | @Inject | HEAD | client | 1000 (default) | `ClientHandshakePacketListenerImplMixin.setGameProfileContext` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]].`<init>` | `(Lnet/minecraft/client/Minecraft;Lnet/minecraft/network/Connection;Lnet/minecraft/client/multiplayer/CommonListenerCookie;)V` | name_only | @Inject | RETURN | client | 999 | `ClientPacketListenerMixin.initAddon` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]].`handleLogin` | `(Lnet/minecraft/network/protocol/game/ClientboundLoginPacket;)V` | name_only | @Inject | RETURN | client | 999 | `ClientPacketListenerMixin.handleServerPlayReady` |
| [[40-Interfaces/net.minecraft.commands.Commands|Commands]].`<init>` | `(Lnet/minecraft/commands/Commands$CommandSelection;Lnet/minecraft/commands/CommandBuildContext;)V` | name_only | @Inject | INVOKE `Lnet/minecraft/server/commands/BanIpCommands;register(Lcom/mojang/brigadier/CommandDispatcher;)V` (exact) | both | 1000 (default) | `CommandsMixin.init` |
| [[40-Interfaces/net.minecraft.network.Connection|Connection]].`<init>` | `(Lnet/minecraft/network/protocol/PacketFlow;)V` | name_only | @Inject | RETURN | both | 1000 (default) | `ConnectionMixin.initAddedFields` |
| [[40-Interfaces/net.minecraft.network.Connection|Connection]].`channelInactive` | `(Lio/netty/channel/ChannelHandlerContext;)V` | name_only | @Inject | HEAD | both | 1000 (default) | `ConnectionMixin.disconnectAddon` |
| [[40-Interfaces/net.minecraft.network.Connection|Connection]].`handleDisconnection` | `()V` | name_only | @Inject | INVOKE `Lnet/minecraft/network/PacketListener;onDisconnect(Lnet/minecraft/network/DisconnectionDetails;)V` (exact) | both | 1000 (default) | `ConnectionMixin.disconnectAddon` |
| [[40-Interfaces/net.minecraft.network.Connection|Connection]].`sendPacket` | `(Lnet/minecraft/network/protocol/Packet;Lio/netty/channel/ChannelFutureListener;Z)V` | name_only | @Inject | FIELD `Lnet/minecraft/network/Connection;sentPackets:I` (exact) | both | 1000 (default) | `ConnectionMixin.checkPacket` |
| [[40-Interfaces/net.minecraft.network.Connection|Connection]].`setupInboundProtocol` | `(Lnet/minecraft/network/ProtocolInfo;Lnet/minecraft/network/PacketListener;)V` | name_only | @ModifyArg | INVOKE `Lio/netty/channel/Channel;writeAndFlush(Ljava/lang/Object;)Lio/netty/channel/ChannelFuture;` (exact) | both | 1000 (default) | `ConnectionMixin.injectFabricPacketSlitterHandlerInbound` |
| [[40-Interfaces/net.minecraft.network.Connection|Connection]].`setupOutboundProtocol` | `(Lnet/minecraft/network/ProtocolInfo;)V` | name_only | @ModifyArg | INVOKE `Lio/netty/channel/Channel;writeAndFlush(Ljava/lang/Object;)Lio/netty/channel/ChannelFuture;` (exact) | both | 1000 (default) | `ConnectionMixin.injectFabricPacketSlitterHandlerOutbound` |
| [[40-Interfaces/net.minecraft.network.Connection|Connection]].`validateListener` | `(Lnet/minecraft/network/ProtocolInfo;Lnet/minecraft/network/PacketListener;)V` | name_only | @Inject | HEAD | both | 1000 (default) | `ConnectionMixin.unwatchAddon` |
| [[40-Interfaces/net.minecraft.network.PacketDecoder|PacketDecoder]].`decode` | `(Lio/netty/channel/ChannelHandlerContext;Lio/netty/buffer/ByteBuf;Ljava/util/List;)V` | name_only | @WrapMethod | - | both | 500 | `PacketDecoderMixin.wrapWithContext` |
| [[40-Interfaces/net.minecraft.network.PacketEncoder|PacketEncoder]].`encode` | `(Lio/netty/channel/ChannelHandlerContext;Lnet/minecraft/network/protocol/Packet;Lio/netty/buffer/ByteBuf;)V` | exact | @Inject | HEAD | both | 500 | `PacketEncoderMixin.handlePassthroughPacket` |
| [[40-Interfaces/net.minecraft.network.PacketEncoder|PacketEncoder]].`encode` | `(Lio/netty/channel/ChannelHandlerContext;Lnet/minecraft/network/protocol/Packet;Lio/netty/buffer/ByteBuf;)V` | exact | @WrapMethod | - | both | 500 | `PacketEncoderMixin.wrapWithContext` |
| [[40-Interfaces/net.minecraft.network.codec.IdDispatchCodec|IdDispatchCodec]].`decode` | `(Lio/netty/buffer/ByteBuf;)Ljava/lang/Object;` | exact | @Inject | HEAD | both | 1000 (default) | `IdDispatchCodecMixin.decode` |
| [[40-Interfaces/net.minecraft.network.codec.IdDispatchCodec|IdDispatchCodec]].`decode` | `(Lio/netty/buffer/ByteBuf;)Ljava/lang/Object;` | exact | @Inject | NEW `(Ljava/lang/String;Ljava/lang/Throwable;)Lio/netty/handler/codec/DecoderException;` (exact) | both | 1000 (default) | `IdDispatchCodecMixin.decode` |
| [[40-Interfaces/net.minecraft.network.codec.IdDispatchCodec|IdDispatchCodec]].`encode` | `(Lio/netty/buffer/ByteBuf;Ljava/lang/Object;)V` | exact | @Inject | NEW `(Ljava/lang/String;Ljava/lang/Throwable;)Lio/netty/handler/codec/EncoderException;` (exact) | both | 1000 (default) | `IdDispatchCodecMixin.encode` |
| [[40-Interfaces/net.minecraft.network.protocol.BundlePacket|BundlePacket]].`<init>` | `(Ljava/lang/Iterable;)V` | name_only | @ModifyVariable | HEAD | both | 1000 (default) | `BundlePacketMixin.flattenBundlePackets` |
| [[40-Interfaces/net.minecraft.network.protocol.common.ClientboundCustomPayloadPacket|ClientboundCustomPayloadPacket]].`<clinit>` | `()V` | exact | @WrapOperation | INVOKE `Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload;codec(Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload$FallbackProvider;Ljava/util/List;)Lnet/minecraft/network/codec/StreamCodec;` (exact) | both | 1000 (default) | `ClientboundCustomPayloadPacketMixin.wrapPlayCodec` |
| [[40-Interfaces/net.minecraft.network.protocol.common.ClientboundCustomPayloadPacket|ClientboundCustomPayloadPacket]].`<clinit>` | `()V` | exact | @WrapOperation | INVOKE `Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload;codec(Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload$FallbackProvider;Ljava/util/List;)Lnet/minecraft/network/codec/StreamCodec;` (exact) | both | 1000 (default) | `ClientboundCustomPayloadPacketMixin.wrapConfigCodec` |
| [[40-Interfaces/net.minecraft.network.protocol.common.ServerboundCustomPayloadPacket|ServerboundCustomPayloadPacket]].`<clinit>` | `()V` | exact | @WrapOperation | INVOKE `Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload;codec(Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload$FallbackProvider;Ljava/util/List;)Lnet/minecraft/network/codec/StreamCodec;` (exact) | both | 1000 (default) | `ServerboundCustomPayloadPacketMixin.wrapCodec` |
| [[40-Interfaces/net.minecraft.network.protocol.common.custom.CustomPacketPayload_1|CustomPacketPayload$1]].`decode` | `(Lnet/minecraft/network/FriendlyByteBuf;)Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload;` | exact | @WrapOperation | INVOKE `Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload$1;findCodec(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/network/codec/StreamCodec;` (exact) | both | 1000 (default) | `CustomPayloadStreamCodecMixin.wrapGetCodec` |
| [[40-Interfaces/net.minecraft.network.protocol.common.custom.CustomPacketPayload_1|CustomPacketPayload$1]].`writeCap` | `(Lnet/minecraft/network/FriendlyByteBuf;Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload$Type;Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload;)V` | exact | @WrapOperation | INVOKE `Lnet/minecraft/network/protocol/common/custom/CustomPacketPayload$1;findCodec(Lnet/minecraft/resources/Identifier;)Lnet/minecraft/network/codec/StreamCodec;` (exact) | both | 1000 (default) | `CustomPayloadStreamCodecMixin.wrapGetCodec` |
| [[40-Interfaces/net.minecraft.network.protocol.login.ClientboundCustomQueryPacket|ClientboundCustomQueryPacket]].`readPayload` | `(Lnet/minecraft/resources/Identifier;Lnet/minecraft/network/FriendlyByteBuf;)Lnet/minecraft/network/protocol/login/custom/CustomQueryPayload;` | name_only | @Inject | HEAD | both | 1000 (default) | `ClientboundCustomQueryPacketMixin.readPayload` |
| [[40-Interfaces/net.minecraft.network.protocol.login.ServerboundCustomQueryAnswerPacket|ServerboundCustomQueryAnswerPacket]].`readPayload` | `(ILnet/minecraft/network/FriendlyByteBuf;)Lnet/minecraft/network/protocol/login/custom/CustomQueryAnswerPayload;` | name_only | @Inject | HEAD | both | 1000 (default) | `ServerboundCustomQueryAnswerPacketMixin.readResponse` |
| [[40-Interfaces/net.minecraft.server.commands.DebugConfigCommand|DebugConfigCommand]].`unconfig` | `(Lnet/minecraft/commands/CommandSourceStack;Ljava/util/UUID;)I` | name_only | @Redirect | INVOKE `Lnet/minecraft/server/network/ServerConfigurationPacketListenerImpl;returnToWorld()V` (exact) | both | 1000 (default) | `DebugConfigCommandMixin.sendConfigurations` |
| [[40-Interfaces/net.minecraft.server.level.ServerEntity|ServerEntity]].`addPairing` | `(Lnet/minecraft/server/level/ServerPlayer;)V` | name_only | @Inject | TAIL | both | 1000 (default) | `ServerEntityMixin.onStartTracking` |
| [[40-Interfaces/net.minecraft.server.level.ServerEntity|ServerEntity]].`removePairing` | `(Lnet/minecraft/server/level/ServerPlayer;)V` | name_only | @Inject | HEAD | both | 1000 (default) | `ServerEntityMixin.onStopTracking` |
| [[40-Interfaces/net.minecraft.server.network.ServerCommonPacketListenerImpl|ServerCommonPacketListenerImpl]].`handleCustomPayload` | `(Lnet/minecraft/network/protocol/common/ServerboundCustomPayloadPacket;)V` | name_only | @Inject | HEAD | both | 1000 (default) | `ServerCommonPacketListenerImplMixin.handleCustomPayloadReceivedAsync` |
| [[40-Interfaces/net.minecraft.server.network.ServerCommonPacketListenerImpl|ServerCommonPacketListenerImpl]].`handlePong` | `(Lnet/minecraft/network/protocol/common/ServerboundPongPacket;)V` | name_only | @Inject | HEAD | both | 1000 (default) | `ServerCommonPacketListenerImplMixin.onPlayPong` |
| [[40-Interfaces/net.minecraft.server.network.ServerConfigurationPacketListenerImpl|ServerConfigurationPacketListenerImpl]].`<init>` | `(Lnet/minecraft/server/MinecraftServer;Lnet/minecraft/network/Connection;Lnet/minecraft/server/network/CommonListenerCookie;)V` | name_only | @Inject | RETURN | both | 900 | `ServerConfigurationPacketListenerImplMixin.initAddon` |
| [[40-Interfaces/net.minecraft.server.network.ServerConfigurationPacketListenerImpl|ServerConfigurationPacketListenerImpl]].`handleConfigurationFinished` | `(Lnet/minecraft/network/protocol/configuration/ServerboundFinishConfigurationPacket;)V` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/network/RegistryFriendlyByteBuf;decorator(Lnet/minecraft/core/RegistryAccess;)Ljava/util/function/Function;` (exact) | both | 900 | `ServerConfigurationPacketListenerImplMixin.bindChannelInfo` |
| [[40-Interfaces/net.minecraft.server.network.ServerConfigurationPacketListenerImpl|ServerConfigurationPacketListenerImpl]].`startConfiguration` | `()V` | name_only | @Inject | HEAD | both | 900 | `ServerConfigurationPacketListenerImplMixin.onClientReady` |
| [[40-Interfaces/net.minecraft.server.network.ServerGamePacketListenerImpl|ServerGamePacketListenerImpl]].`<init>` | `(Lnet/minecraft/server/MinecraftServer;Lnet/minecraft/network/Connection;Lnet/minecraft/server/level/ServerPlayer;Lnet/minecraft/server/network/CommonListenerCookie;)V` | name_only | @Inject | RETURN | both | 999 | `ServerGamePacketListenerImplMixin.initAddon` |
| [[40-Interfaces/net.minecraft.server.network.ServerGamePacketListenerImpl|ServerGamePacketListenerImpl]].`handleConfigurationAcknowledged` | `(Lnet/minecraft/network/protocol/game/ServerboundConfigurationAcknowledgedPacket;)V` | name_only | @WrapOperation | INVOKE `Lnet/minecraft/network/Connection;setupInboundProtocol(Lnet/minecraft/network/ProtocolInfo;Lnet/minecraft/network/PacketListener;)V` (exact) | both | 999 | `ServerGamePacketListenerImplMixin.onAcknowledgeReconfiguration` |
| [[40-Interfaces/net.minecraft.server.network.ServerGamePacketListenerImpl|ServerGamePacketListenerImpl]].`handleCustomPayload` | `(Lnet/minecraft/network/protocol/common/ServerboundCustomPayloadPacket;)V` | name_only | @Inject | HEAD | both | 999 | `ServerGamePacketListenerImplMixin.handleCustomPayloadReceivedAsync` |
| [[40-Interfaces/net.minecraft.server.network.ServerLoginPacketListenerImpl|ServerLoginPacketListenerImpl]].`<init>` | `(Lnet/minecraft/server/MinecraftServer;Lnet/minecraft/network/Connection;Z)V` | name_only | @Inject | RETURN | both | 1000 (default) | `ServerLoginPacketListenerImplMixin.initAddon` |
| [[40-Interfaces/net.minecraft.server.network.ServerLoginPacketListenerImpl|ServerLoginPacketListenerImpl]].`finishLoginAndWaitForClient` | `(Lcom/mojang/authlib/GameProfile;)V` | name_only | @Inject | HEAD | both | 1000 (default) | `ServerLoginPacketListenerImplMixin.storeGameProfileContext` |
| [[40-Interfaces/net.minecraft.server.network.ServerLoginPacketListenerImpl|ServerLoginPacketListenerImpl]].`handleCustomQueryPacket` | `(Lnet/minecraft/network/protocol/login/ServerboundCustomQueryAnswerPacket;)V` | name_only | @Inject | HEAD | both | 1000 (default) | `ServerLoginPacketListenerImplMixin.handleCustomPayloadReceivedAsync` |
| [[40-Interfaces/net.minecraft.server.network.ServerLoginPacketListenerImpl|ServerLoginPacketListenerImpl]].`tick` | `()V` | name_only | @Redirect | INVOKE `Lnet/minecraft/server/network/ServerLoginPacketListenerImpl;verifyLoginAndFinishConnectionSetup(Lcom/mojang/authlib/GameProfile;)V` (exact) | both | 1000 (default) | `ServerLoginPacketListenerImplMixin.handlePlayerJoin` |
| [[40-Interfaces/net.minecraft.server.network.ServerLoginPacketListenerImpl|ServerLoginPacketListenerImpl]].`verifyLoginAndFinishConnectionSetup` | `(Lcom/mojang/authlib/GameProfile;)V` | name_only | @Redirect | INVOKE `Lnet/minecraft/server/MinecraftServer;getCompressionThreshold()I` (exact) | both | 1000 (default) | `ServerLoginPacketListenerImplMixin.removeLateCompressionPacketSending` |
| [[40-Interfaces/net.minecraft.server.players.PlayerList|PlayerList]].`placeNewPlayer` | `(Lnet/minecraft/network/Connection;Lnet/minecraft/server/level/ServerPlayer;Lnet/minecraft/server/network/CommonListenerCookie;)V` | name_only | @Inject | INVOKE `Lnet/minecraft/network/protocol/game/ClientboundPlayerAbilitiesPacket;<init>(Lnet/minecraft/world/entity/player/Abilities;)V` (exact) | both | 1000 (default) | `PlayerListMixin.handlePlayerConnection` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.client.networking.v1.ClientConfigurationConnectionEvents|ClientConfigurationConnectionEvents]] (class, 5 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.networking.v1.ClientConfigurationNetworking|ClientConfigurationNetworking]] (class, 11 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.networking.v1.ClientLoginConnectionEvents|ClientLoginConnectionEvents]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.networking.v1.ClientLoginNetworking|ClientLoginNetworking]] (class, 5 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.networking.v1.ClientPlayConnectionEvents|ClientPlayConnectionEvents]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.networking.v1.ClientPlayNetworking|ClientPlayNetworking]] (class, 12 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.networking.v1.ServerboundConfigurationChannelEvents|ServerboundConfigurationChannelEvents]] (class, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.networking.v1.ServerboundPlayChannelEvents|ServerboundPlayChannelEvents]] (class, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.networking.v1.ClientboundConfigurationChannelEvents|ClientboundConfigurationChannelEvents]] (class, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.networking.v1.ClientboundPlayChannelEvents|ClientboundPlayChannelEvents]] (class, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.networking.v1.EntityTrackingEvents|EntityTrackingEvents]] (class, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.networking.v1.FabricServerConfigurationPacketListenerImpl|FabricServerConfigurationPacketListenerImpl]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.networking.v1.FriendlyByteBufs|FriendlyByteBufs]] (class, 13 members)
- [[40-Interfaces/net.fabricmc.fabric.api.networking.v1.LoginPacketSender|LoginPacketSender]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.networking.v1.PacketSender|PacketSender]] (interface, 6 members)
- [[40-Interfaces/net.fabricmc.fabric.api.networking.v1.PayloadTypeRegistry|PayloadTypeRegistry]] (interface, 7 members)
- [[40-Interfaces/net.fabricmc.fabric.api.networking.v1.PlayerLookup|PlayerLookup]] (class, 8 members)
- [[40-Interfaces/net.fabricmc.fabric.api.networking.v1.ServerConfigurationConnectionEvents|ServerConfigurationConnectionEvents]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.networking.v1.ServerConfigurationNetworking|ServerConfigurationNetworking]] (class, 14 members)
- [[40-Interfaces/net.fabricmc.fabric.api.networking.v1.ServerLoginConnectionEvents|ServerLoginConnectionEvents]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.networking.v1.ServerLoginNetworking|ServerLoginNetworking]] (class, 7 members)
- [[40-Interfaces/net.fabricmc.fabric.api.networking.v1.ServerPlayConnectionEvents|ServerPlayConnectionEvents]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.networking.v1.ServerPlayNetworking|ServerPlayNetworking]] (class, 19 members)
- [[40-Interfaces/net.fabricmc.fabric.api.networking.v1.context.PacketContext|PacketContext]] (interface, 15 members)
- [[40-Interfaces/net.fabricmc.fabric.api.networking.v1.context.PacketContextProvider|PacketContextProvider]] (interface, 1 members)

## What this establishes, and does not

- Injection targets, points and API signatures are `direct_reference`: read from the class files.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- How two injections compose is `executed_transformation` evidence in [[30-Mechanisms/Transformation_Tests]], not established per module.
- Nothing here is `observed`. No game ran.
