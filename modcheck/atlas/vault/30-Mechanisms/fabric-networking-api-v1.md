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

## Vanilla types this module modifies (mixins)

| vanilla type | method | how | environment | mixin |
|---|---|---|---|---|
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientCommonPacketListenerImpl|ClientCommonPacketListenerImpl]] | `handleCustomPayload(Lnet/minecraft/network/protocol/common/ClientboundCustomPayloadPacket;)V` | injects_into `@Inject at HEAD` | client | `ClientCommonPacketListenerImplMixin.onCustomPayload` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientConfigurationPacketListenerImpl|ClientConfigurationPacketListenerImpl]] | `<init>` | injects_into `@Inject at RETURN` | client | `ClientConfigurationPacketListenerImplMixin.initAddon` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientConfigurationPacketListenerImpl|ClientConfigurationPacketListenerImpl]] | `handleConfigurationFinished` | injects_into `@Inject at NEW (Lnet/minecraft/client/Minecraft;Lnet/minecraft/network/Connection;Lnet/minecraft/client/multiplayer/CommonListenerCookie;)Lnet/minecraft/client/multiplayer/ClientPacketListener;` | client | `ClientConfigurationPacketListenerImplMixin.handleComplete` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientHandshakePacketListenerImpl|ClientHandshakePacketListenerImpl]] | `<init>` | injects_into `@Inject at RETURN` | client | `ClientHandshakePacketListenerImplMixin.initAddon` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientHandshakePacketListenerImpl|ClientHandshakePacketListenerImpl]] | `handleCustomQuery` | injects_into `@Inject at INVOKE Ljava/util/function/Consumer;accept(Ljava/lang/Object;)V` | client | `ClientHandshakePacketListenerImplMixin.handleQueryRequest` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientHandshakePacketListenerImpl|ClientHandshakePacketListenerImpl]] | `handleLoginFinished` | injects_into `@Inject at HEAD` | client | `ClientHandshakePacketListenerImplMixin.setGameProfileContext` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]] | `<init>` | injects_into `@Inject at RETURN` | client | `ClientPacketListenerMixin.initAddon` |
| [[40-Interfaces/net.minecraft.client.multiplayer.ClientPacketListener|ClientPacketListener]] | `handleLogin` | injects_into `@Inject at RETURN` | client | `ClientPacketListenerMixin.handleServerPlayReady` |
| [[40-Interfaces/net.minecraft.commands.Commands|Commands]] | `<init>` | injects_into `@Inject at INVOKE Lnet/minecraft/server/commands/BanIpCommands;register(Lcom/mojang/brigadier/CommandDispatcher;)V` | both | `CommandsMixin.init` |
| [[40-Interfaces/net.minecraft.network.Connection|Connection]] | `<init>` | injects_into `@Inject at RETURN` | both | `ConnectionMixin.initAddedFields` |
| [[40-Interfaces/net.minecraft.network.Connection|Connection]] | `channelInactive` | injects_into `@Inject at HEAD` | both | `ConnectionMixin.disconnectAddon` |
| [[40-Interfaces/net.minecraft.network.Connection|Connection]] | `handleDisconnection` | injects_into `@Inject at INVOKE Lnet/minecraft/network/PacketListener;onDisconnect(Lnet/minecraft/network/DisconnectionDetails;)V` | both | `ConnectionMixin.disconnectAddon` |
| [[40-Interfaces/net.minecraft.network.Connection|Connection]] | `sendPacket` | injects_into `@Inject at FIELD Lnet/minecraft/network/Connection;sentPackets:I` | both | `ConnectionMixin.checkPacket` |
| [[40-Interfaces/net.minecraft.network.Connection|Connection]] | `setupInboundProtocol` | injects_into `@ModifyArg at INVOKE Lio/netty/channel/Channel;writeAndFlush(Ljava/lang/Object;)Lio/netty/channel/ChannelFuture;` | both | `ConnectionMixin.injectFabricPacketSlitterHandlerInbound` |
| [[40-Interfaces/net.minecraft.network.Connection|Connection]] | `setupOutboundProtocol` | injects_into `@ModifyArg at INVOKE Lio/netty/channel/Channel;writeAndFlush(Ljava/lang/Object;)Lio/netty/channel/ChannelFuture;` | both | `ConnectionMixin.injectFabricPacketSlitterHandlerOutbound` |
| [[40-Interfaces/net.minecraft.network.Connection|Connection]] | `validateListener` | injects_into `@Inject at HEAD` | both | `ConnectionMixin.unwatchAddon` |
| [[40-Interfaces/net.minecraft.network.PacketEncoder|PacketEncoder]] | `encode(Lio/netty/channel/ChannelHandlerContext;Lnet/minecraft/network/protocol/Packet;Lio/netty/buffer/ByteBuf;)V` | injects_into `@Inject at HEAD` | both | `PacketEncoderMixin.handlePassthroughPacket` |
| [[40-Interfaces/net.minecraft.network.codec.IdDispatchCodec|IdDispatchCodec]] | `decode(Lio/netty/buffer/ByteBuf;)Ljava/lang/Object;` | injects_into `@Inject at HEAD` | both | `IdDispatchCodecMixin.decode` |
| [[40-Interfaces/net.minecraft.network.codec.IdDispatchCodec|IdDispatchCodec]] | `decode(Lio/netty/buffer/ByteBuf;)Ljava/lang/Object;` | injects_into `@Inject at NEW (Ljava/lang/String;Ljava/lang/Throwable;)Lio/netty/handler/codec/DecoderException;` | both | `IdDispatchCodecMixin.decode` |
| [[40-Interfaces/net.minecraft.network.codec.IdDispatchCodec|IdDispatchCodec]] | `encode(Lio/netty/buffer/ByteBuf;Ljava/lang/Object;)V` | injects_into `@Inject at NEW (Ljava/lang/String;Ljava/lang/Throwable;)Lio/netty/handler/codec/EncoderException;` | both | `IdDispatchCodecMixin.encode` |
| [[40-Interfaces/net.minecraft.network.protocol.BundlePacket|BundlePacket]] | `<init>` | injects_into `@ModifyVariable at HEAD` | both | `BundlePacketMixin.flattenBundlePackets` |
| [[40-Interfaces/net.minecraft.network.protocol.login.ClientboundCustomQueryPacket|ClientboundCustomQueryPacket]] | `readPayload` | injects_into `@Inject at HEAD` | both | `ClientboundCustomQueryPacketMixin.readPayload` |
| [[40-Interfaces/net.minecraft.network.protocol.login.ServerboundCustomQueryAnswerPacket|ServerboundCustomQueryAnswerPacket]] | `readPayload` | injects_into `@Inject at HEAD` | both | `ServerboundCustomQueryAnswerPacketMixin.readResponse` |
| [[40-Interfaces/net.minecraft.server.commands.DebugConfigCommand|DebugConfigCommand]] | `unconfig` | wraps `@Redirect at INVOKE Lnet/minecraft/server/network/ServerConfigurationPacketListenerImpl;returnToWorld()V` | both | `DebugConfigCommandMixin.sendConfigurations` |
| [[40-Interfaces/net.minecraft.server.level.ServerEntity|ServerEntity]] | `addPairing` | injects_into `@Inject at TAIL` | both | `ServerEntityMixin.onStartTracking` |
| [[40-Interfaces/net.minecraft.server.level.ServerEntity|ServerEntity]] | `removePairing` | injects_into `@Inject at HEAD` | both | `ServerEntityMixin.onStopTracking` |
| [[40-Interfaces/net.minecraft.server.network.ServerCommonPacketListenerImpl|ServerCommonPacketListenerImpl]] | `handleCustomPayload` | injects_into `@Inject at HEAD` | both | `ServerCommonPacketListenerImplMixin.handleCustomPayloadReceivedAsync` |
| [[40-Interfaces/net.minecraft.server.network.ServerCommonPacketListenerImpl|ServerCommonPacketListenerImpl]] | `handlePong` | injects_into `@Inject at HEAD` | both | `ServerCommonPacketListenerImplMixin.onPlayPong` |
| [[40-Interfaces/net.minecraft.server.network.ServerConfigurationPacketListenerImpl|ServerConfigurationPacketListenerImpl]] | `<init>` | injects_into `@Inject at RETURN` | both | `ServerConfigurationPacketListenerImplMixin.initAddon` |
| [[40-Interfaces/net.minecraft.server.network.ServerConfigurationPacketListenerImpl|ServerConfigurationPacketListenerImpl]] | `startConfiguration` | injects_into `@Inject at HEAD` | both | `ServerConfigurationPacketListenerImplMixin.onClientReady` |
| [[40-Interfaces/net.minecraft.server.network.ServerGamePacketListenerImpl|ServerGamePacketListenerImpl]] | `<init>` | injects_into `@Inject at RETURN` | both | `ServerGamePacketListenerImplMixin.initAddon` |
| [[40-Interfaces/net.minecraft.server.network.ServerGamePacketListenerImpl|ServerGamePacketListenerImpl]] | `handleCustomPayload` | injects_into `@Inject at HEAD` | both | `ServerGamePacketListenerImplMixin.handleCustomPayloadReceivedAsync` |
| [[40-Interfaces/net.minecraft.server.network.ServerLoginPacketListenerImpl|ServerLoginPacketListenerImpl]] | `<init>` | injects_into `@Inject at RETURN` | both | `ServerLoginPacketListenerImplMixin.initAddon` |
| [[40-Interfaces/net.minecraft.server.network.ServerLoginPacketListenerImpl|ServerLoginPacketListenerImpl]] | `finishLoginAndWaitForClient` | injects_into `@Inject at HEAD` | both | `ServerLoginPacketListenerImplMixin.storeGameProfileContext` |
| [[40-Interfaces/net.minecraft.server.network.ServerLoginPacketListenerImpl|ServerLoginPacketListenerImpl]] | `handleCustomQueryPacket` | injects_into `@Inject at HEAD` | both | `ServerLoginPacketListenerImplMixin.handleCustomPayloadReceivedAsync` |
| [[40-Interfaces/net.minecraft.server.network.ServerLoginPacketListenerImpl|ServerLoginPacketListenerImpl]] | `tick` | wraps `@Redirect at INVOKE Lnet/minecraft/server/network/ServerLoginPacketListenerImpl;verifyLoginAndFinishConnectionSetup(Lcom/mojang/authlib/GameProfile;)V` | both | `ServerLoginPacketListenerImplMixin.handlePlayerJoin` |
| [[40-Interfaces/net.minecraft.server.network.ServerLoginPacketListenerImpl|ServerLoginPacketListenerImpl]] | `verifyLoginAndFinishConnectionSetup` | wraps `@Redirect at INVOKE Lnet/minecraft/server/MinecraftServer;getCompressionThreshold()I` | both | `ServerLoginPacketListenerImplMixin.removeLateCompressionPacketSending` |
| [[40-Interfaces/net.minecraft.server.players.PlayerList|PlayerList]] | `placeNewPlayer` | injects_into `@Inject at INVOKE Lnet/minecraft/network/protocol/game/ClientboundPlayerAbilitiesPacket;<init>(Lnet/minecraft/world/entity/player/Abilities;)V` | both | `PlayerListMixin.handlePlayerConnection` |

## API surface

- [[40-Interfaces/net.fabricmc.fabric.api.client.networking.v1.ClientConfigurationConnectionEvents|ClientConfigurationConnectionEvents]] (class, 6 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.networking.v1.ClientConfigurationNetworking|ClientConfigurationNetworking]] (class, 11 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.networking.v1.ClientLoginConnectionEvents|ClientLoginConnectionEvents]] (class, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.networking.v1.ClientLoginNetworking|ClientLoginNetworking]] (class, 5 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.networking.v1.ClientPlayConnectionEvents|ClientPlayConnectionEvents]] (class, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.networking.v1.ClientPlayNetworking|ClientPlayNetworking]] (class, 12 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.networking.v1.ServerboundConfigurationChannelEvents|ServerboundConfigurationChannelEvents]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.client.networking.v1.ServerboundPlayChannelEvents|ServerboundPlayChannelEvents]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.networking.v1.ClientboundConfigurationChannelEvents|ClientboundConfigurationChannelEvents]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.networking.v1.ClientboundPlayChannelEvents|ClientboundPlayChannelEvents]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.networking.v1.EntityTrackingEvents|EntityTrackingEvents]] (class, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.networking.v1.FabricServerConfigurationPacketListenerImpl|FabricServerConfigurationPacketListenerImpl]] (interface, 2 members)
- [[40-Interfaces/net.fabricmc.fabric.api.networking.v1.FriendlyByteBufs|FriendlyByteBufs]] (class, 14 members)
- [[40-Interfaces/net.fabricmc.fabric.api.networking.v1.LoginPacketSender|LoginPacketSender]] (interface, 3 members)
- [[40-Interfaces/net.fabricmc.fabric.api.networking.v1.PacketSender|PacketSender]] (interface, 6 members)
- [[40-Interfaces/net.fabricmc.fabric.api.networking.v1.PayloadTypeRegistry|PayloadTypeRegistry]] (interface, 7 members)
- [[40-Interfaces/net.fabricmc.fabric.api.networking.v1.PlayerLookup|PlayerLookup]] (class, 8 members)
- [[40-Interfaces/net.fabricmc.fabric.api.networking.v1.ServerConfigurationConnectionEvents|ServerConfigurationConnectionEvents]] (class, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.networking.v1.ServerConfigurationNetworking|ServerConfigurationNetworking]] (class, 14 members)
- [[40-Interfaces/net.fabricmc.fabric.api.networking.v1.ServerLoginConnectionEvents|ServerLoginConnectionEvents]] (class, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.networking.v1.ServerLoginNetworking|ServerLoginNetworking]] (class, 7 members)
- [[40-Interfaces/net.fabricmc.fabric.api.networking.v1.ServerPlayConnectionEvents|ServerPlayConnectionEvents]] (class, 4 members)
- [[40-Interfaces/net.fabricmc.fabric.api.networking.v1.ServerPlayNetworking|ServerPlayNetworking]] (class, 19 members)
- [[40-Interfaces/net.fabricmc.fabric.api.networking.v1.context.PacketContext|PacketContext]] (interface, 16 members)
- [[40-Interfaces/net.fabricmc.fabric.api.networking.v1.context.PacketContextProvider|PacketContextProvider]] (interface, 1 members)

## What this establishes, and does not

- Injection targets and API signatures are `direct_reference`: read from the jar.
- Event publication is `static_inference`: a bytecode pattern, labelled as such.
- Nothing here is `observed`. No game ran.
