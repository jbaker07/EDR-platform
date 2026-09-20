---
type: "system"
package: "net.minecraft.server.jsonrpc"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.jsonrpc

95 classes (65 top-level) across 7 packages in the processed jar; 0 changed by Loom processing; 2 hooked by Fabric API.

## Hooked types

- [[40-Interfaces/net.minecraft.server.jsonrpc.methods.GameRulesService_GameRuleUpdate|GameRulesService$GameRuleUpdate]] -- calls:2, injects_into:2 -- by fabric-game-rule-api-v1
- [[40-Interfaces/net.minecraft.server.jsonrpc.methods.InvalidParameterJsonRpcException|InvalidParameterJsonRpcException]] -- calls:1 -- by fabric-game-rule-api-v1

## Declared inventory

### `net.minecraft.server.jsonrpc` (13 top-level)

`Connection`, `IncomingRpcMethod`, `IncomingRpcMethods`, `JsonRPCErrors`, `JsonRPCUtils`, `JsonRpc`, `JsonRpcLogger`, `JsonRpcNotificationService`, `ManagementServer`, `OutgoingRpcMethod`, `OutgoingRpcMethods`, `PendingRpcRequest`, `package-info`

### `net.minecraft.server.jsonrpc.api` (8 top-level)

`MethodInfo`, `ParamInfo`, `PlayerDto`, `ReferenceUtil`, `ResultInfo`, `Schema`, `SchemaComponent`, `package-info`

### `net.minecraft.server.jsonrpc.dataprovider` (2 top-level)

`JsonRpcApiSchema`, `package-info`

### `net.minecraft.server.jsonrpc.internalapi` (18 top-level)

`MinecraftAllowListService`, `MinecraftAllowListServiceImpl`, `MinecraftApi`, `MinecraftBanListService`, `MinecraftBanListServiceImpl`, `MinecraftExecutorService`, `MinecraftExecutorServiceImpl`, `MinecraftGameRuleService`, `MinecraftGameRuleServiceImpl`, `MinecraftOperatorListService`, `MinecraftOperatorListServiceImpl`, `MinecraftPlayerListService`, `MinecraftPlayerListServiceImpl`, `MinecraftServerSettingsService`, `MinecraftServerSettingsServiceImpl`, `MinecraftServerStateService`, `MinecraftServerStateServiceImpl`, `package-info`

### `net.minecraft.server.jsonrpc.methods` (17 top-level)

`AllowlistService`, `BanlistService`, `ClientInfo`, `DiscoveryService`, `EncodeJsonRpcException`, `GameRulesService`, [[40-Interfaces/net.minecraft.server.jsonrpc.methods.InvalidParameterJsonRpcException|InvalidParameterJsonRpcException]], `InvalidRequestJsonRpcException`, `IpBanlistService`, `Message`, `MethodNotFoundJsonRpcException`, `OperatorService`, `PlayerService`, `RemoteRpcErrorException`, `ServerSettingsService`, `ServerStateService`, `package-info`

### `net.minecraft.server.jsonrpc.security` (4 top-level)

`AuthenticationHandler`, `JsonRpcSslContextProvider`, `SecurityConfig`, `package-info`

### `net.minecraft.server.jsonrpc.websocket` (3 top-level)

`JsonToWebSocketEncoder`, `WebSocketToJsonCodec`, `package-info`

