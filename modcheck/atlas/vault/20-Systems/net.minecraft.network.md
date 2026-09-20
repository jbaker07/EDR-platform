---
type: "system"
package: "net.minecraft.network"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network

56 classes (42 top-level) across 1 packages in the processed jar; 2 changed by Loom processing; 15 hooked by Fabric API.

## Hooked types

- [[40-Interfaces/net.minecraft.network.Connection|Connection]] -- calls:23, injects_into:7, reads:1 -- by fabric-data-attachment-api-v1, fabric-events-interaction-v0, fabric-lifecycle-events-v1, fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.ConnectionProtocol|ConnectionProtocol]] -- calls:13, reads:22 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.FriendlyByteBuf|FriendlyByteBuf]] -- calls:59 -- by fabric-networking-api-v1, fabric-registry-sync-v0
- [[40-Interfaces/net.minecraft.network.PacketDecoder|PacketDecoder]] -- wraps:1 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.PacketEncoder|PacketEncoder]] -- injects_into:1, wraps:1 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.PacketProcessor|PacketProcessor]] -- calls:5 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.PacketSendListener|PacketSendListener]] -- calls:1 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.ProtocolInfo|ProtocolInfo]] -- calls:7 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.ProtocolInfo_Details|ProtocolInfo$Details]] -- calls:1 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.ProtocolInfo_DetailsProvider|ProtocolInfo$DetailsProvider]] -- calls:1 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.RegistryFriendlyByteBuf|RegistryFriendlyByteBuf]] -- calls:32 -- by fabric-data-attachment-api-v1, fabric-menu-api-v1, fabric-particles-v1, fabric-recipe-api-v1
- [[40-Interfaces/net.minecraft.network.SkipPacketDecoderException|SkipPacketDecoderException]] -- calls:1 -- by fabric-recipe-api-v1
- [[40-Interfaces/net.minecraft.network.UnconfiguredPipelineHandler_InboundConfigurationTask|UnconfiguredPipelineHandler$InboundConfigurationTask]] -- calls:2 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.UnconfiguredPipelineHandler_OutboundConfigurationTask|UnconfiguredPipelineHandler$OutboundConfigurationTask]] -- calls:2 -- by fabric-networking-api-v1
- [[40-Interfaces/net.minecraft.network.VarInt|VarInt]] -- calls:6 -- by fabric-data-attachment-api-v1, fabric-networking-api-v1

## Declared inventory

### `net.minecraft.network` (42 top-level)

`BandwidthDebugMonitor`, `CipherBase`, `CipherDecoder`, `CipherEncoder`, `ClientboundPacketListener`, `CompressionDecoder`, `CompressionEncoder`, [[40-Interfaces/net.minecraft.network.Connection|Connection]], [[40-Interfaces/net.minecraft.network.ConnectionProtocol|ConnectionProtocol]], `DisconnectionDetails`, [[40-Interfaces/net.minecraft.network.FriendlyByteBuf|FriendlyByteBuf]], `HandlerNames`, `HashedPatchMap`, `HashedStack`, `HiddenByteBuf`, `LocalFrameDecoder`, `LocalFrameEncoder`, `LpVec3`, `MonitoredLocalFrameDecoder`, `PacketBundlePacker`, `PacketBundleUnpacker`, [[40-Interfaces/net.minecraft.network.PacketDecoder|PacketDecoder]], [[40-Interfaces/net.minecraft.network.PacketEncoder|PacketEncoder]], `PacketListener`, [[40-Interfaces/net.minecraft.network.PacketProcessor|PacketProcessor]], [[40-Interfaces/net.minecraft.network.PacketSendListener|PacketSendListener]], [[40-Interfaces/net.minecraft.network.ProtocolInfo|ProtocolInfo]], `ProtocolSwapHandler`, `RateKickingConnection`, [[40-Interfaces/net.minecraft.network.RegistryFriendlyByteBuf|RegistryFriendlyByteBuf]], `ServerboundPacketListener`, [[40-Interfaces/net.minecraft.network.SkipPacketDecoderException|SkipPacketDecoderException]], `SkipPacketEncoderException`, `SkipPacketException`, `TickablePacketListener`, `UnconfiguredPipelineHandler`, `Utf8String`, [[40-Interfaces/net.minecraft.network.VarInt|VarInt]], `VarLong`, `Varint21FrameDecoder`, `Varint21LengthFieldPrepender`, `package-info`

