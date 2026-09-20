---
type: "interface"
fqcn: "net.minecraft.server.players.PlayerList"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.players.PlayerList

System: [[20-Systems/net.minecraft.server.players|net.minecraft.server.players]]

`abstract_class` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getPlayer` | `(Ljava/util/UUID;)Lnet/minecraft/server/level/ServerPlayer;` | exact | invokevirtual@33 in `TestServerConnectionImpl.getServerPlayer` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getPlayers` | `()Ljava/util/List;` | exact | invokevirtual@4 in `PlayerListMixin.hookOnDataPacksReloaded` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| calls | `getPlayers` | `()Ljava/util/List;` | exact | invokevirtual@18 in `PlayerLookup.all` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `broadcastChatMessage` | `(Lnet/minecraft/network/chat/PlayerChatMessage;Lnet/minecraft/commands` | exact | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-message-api-v1|fabric-message-api-v1]] | direct_reference |
| injects_into | `broadcastChatMessage` | `(Lnet/minecraft/network/chat/PlayerChatMessage;Lnet/minecraft/server/l` | exact | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-message-api-v1|fabric-message-api-v1]] | direct_reference |
| injects_into | `broadcastSystemMessage` | `(Lnet/minecraft/network/chat/Component;Ljava/util/function/Function;Z)` | exact | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-message-api-v1|fabric-message-api-v1]] | direct_reference |
| injects_into | `placeNewPlayer` | `(Lnet/minecraft/network/Connection;Lnet/minecraft/server/level/ServerP` | name_only | @Inject at ['RETURN'] | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| injects_into | `placeNewPlayer` | `(Lnet/minecraft/network/Connection;Lnet/minecraft/server/level/ServerP` | name_only | @Inject at ['NEW'] | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `placeNewPlayer` | `(Lnet/minecraft/network/Connection;Lnet/minecraft/server/level/ServerP` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| injects_into | `reloadResources` | `()V` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |
| injects_into | `remove` | `(Lnet/minecraft/server/level/ServerPlayer;)V` | name_only | @Inject at ['HEAD'] | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| injects_into | `respawn` | `(Lnet/minecraft/server/level/ServerPlayer;ZLnet/minecraft/world/entity` | name_only | @Inject at ['TAIL'] | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |
| reads | `server` | `Lnet/minecraft/server/MinecraftServer;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-message-api-v1|fabric-message-api-v1]] | declared |

## Declared members (24 fields, 66 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final USERBANLIST_FILE : Ljava/io/File;
public static final IPBANLIST_FILE : Ljava/io/File;
public static final OPLIST_FILE : Ljava/io/File;
public static final WHITELIST_FILE : Ljava/io/File;
public static final CHAT_FILTERED_FULL : Lnet/minecraft/network/chat/Component;
public static final DUPLICATE_LOGIN_DISCONNECT_MESSAGE : Lnet/minecraft/network/chat/Component;
private static final LOGGER : Lorg/slf4j/Logger;
private static final SEND_PLAYER_INFO_INTERVAL : I
private static final BAN_DATE_FORMAT : Ljava/text/SimpleDateFormat;
private final server : Lnet/minecraft/server/MinecraftServer;
private final players : Ljava/util/List;
private final playersByUUID : Ljava/util/Map;
private final playerPermissions : Ljava/util/Map;
private final bans : Lnet/minecraft/server/players/UserBanList;
private final ipBans : Lnet/minecraft/server/players/IpBanList;
private final ops : Lnet/minecraft/server/players/ServerOpList;
private final whitelist : Lnet/minecraft/server/players/UserWhiteList;
private final stats : Ljava/util/Map;
private final advancements : Ljava/util/Map;
private final playerIo : Lnet/minecraft/world/level/storage/PlayerDataStorage;
private final registries : Lnet/minecraft/core/LayeredRegistryAccess;
private viewDistance : I
private simulationDistance : I
private sendAllPlayerInfoIn : I
public <init>(Lnet/minecraft/server/MinecraftServer;Lnet/minecraft/core/LayeredRegistryAccess;Lnet/minecraft/world/level/storage/PlayerDataStorage;Lnet/minecraft/server/notifications/NotificationService;)V
public placeNewPlayer(Lnet/minecraft/network/Connection;Lnet/minecraft/server/level/ServerPlayer;Lnet/minecraft/server/network/CommonListenerCookie;)V
protected updateEntireScoreboard(Lnet/minecraft/server/ServerScoreboard;Lnet/minecraft/server/level/ServerPlayer;)V
public addWorldborderListener(Lnet/minecraft/server/level/ServerLevel;)V
public loadPlayerData(Lnet/minecraft/server/players/NameAndId;)Ljava/util/Optional;
protected save(Lnet/minecraft/server/level/ServerPlayer;)V
public remove(Lnet/minecraft/server/level/ServerPlayer;)V
public canPlayerLogin(Ljava/net/SocketAddress;Lnet/minecraft/server/players/NameAndId;)Lnet/minecraft/network/chat/Component;
public disconnectAllPlayersWithProfile(Ljava/util/UUID;)Z
public respawn(Lnet/minecraft/server/level/ServerPlayer;ZLnet/minecraft/world/entity/Entity$RemovalReason;)Lnet/minecraft/server/level/ServerPlayer;
public sendActivePlayerEffects(Lnet/minecraft/server/level/ServerPlayer;)V
public sendActiveEffects(Lnet/minecraft/world/entity/LivingEntity;Lnet/minecraft/server/network/ServerGamePacketListenerImpl;)V
public sendPlayerPermissionLevel(Lnet/minecraft/server/level/ServerPlayer;)V
public tick()V
public broadcastAll(Lnet/minecraft/network/protocol/Packet;)V
public broadcastAll(Lnet/minecraft/network/protocol/Packet;Lnet/minecraft/resources/ResourceKey;)V
public broadcastSystemToTeam(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/network/chat/Component;)V
public broadcastSystemToAllExceptTeam(Lnet/minecraft/world/entity/player/Player;Lnet/minecraft/network/chat/Component;)V
public getPlayerNamesArray()[Ljava/lang/String;
public getBans()Lnet/minecraft/server/players/UserBanList;
public getIpBans()Lnet/minecraft/server/players/IpBanList;
public op(Lnet/minecraft/server/players/NameAndId;)V
public op(Lnet/minecraft/server/players/NameAndId;Ljava/util/Optional;Ljava/util/Optional;)V
public deop(Lnet/minecraft/server/players/NameAndId;)V
private sendPlayerPermissionLevel(Lnet/minecraft/server/level/ServerPlayer;Lnet/minecraft/server/permissions/LevelBasedPermissionSet;)V
public isWhiteListed(Lnet/minecraft/server/players/NameAndId;)Z
public isOp(Lnet/minecraft/server/players/NameAndId;)Z
public getPlayerByName(Ljava/lang/String;)Lnet/minecraft/server/level/ServerPlayer;
public broadcast(Lnet/minecraft/world/entity/player/Player;DDDDLnet/minecraft/resources/ResourceKey;Lnet/minecraft/network/protocol/Packet;)V
public saveAll()V
public getWhiteList()Lnet/minecraft/server/players/UserWhiteList;
public getWhiteListNames()[Ljava/lang/String;
public getOps()Lnet/minecraft/server/players/ServerOpList;
public getOpNames()[Ljava/lang/String;
public reloadWhiteList()V
public sendLevelInfo(Lnet/minecraft/server/level/ServerPlayer;Lnet/minecraft/server/level/ServerLevel;)V
public sendAllPlayerInfo(Lnet/minecraft/server/level/ServerPlayer;)V
public getPlayerCount()I
public getMaxPlayers()I
public isUsingWhitelist()Z
public getPlayersWithAddress(Ljava/lang/String;)Ljava/util/List;
public getViewDistance()I
public getSimulationDistance()I
public getServer()Lnet/minecraft/server/MinecraftServer;
public removeAll()V
public broadcastSystemMessage(Lnet/minecraft/network/chat/Component;Z)V
public broadcastSystemMessage(Lnet/minecraft/network/chat/Component;Ljava/util/function/Function;Z)V
public broadcastChatMessage(Lnet/minecraft/network/chat/PlayerChatMessage;Lnet/minecraft/commands/CommandSourceStack;Lnet/minecraft/network/chat/ChatType$Bound;)V
public broadcastChatMessage(Lnet/minecraft/network/chat/PlayerChatMessage;Lnet/minecraft/server/level/ServerPlayer;Lnet/minecraft/network/chat/ChatType$Bound;)V
private broadcastChatMessage(Lnet/minecraft/network/chat/PlayerChatMessage;Ljava/util/function/Predicate;Lnet/minecraft/server/level/ServerPlayer;Lnet/minecraft/network/chat/ChatType$Bound;)V
private verifyChatTrusted(Lnet/minecraft/network/chat/PlayerChatMessage;)Z
public getPlayerStats(Lnet/minecraft/world/entity/player/Player;)Lnet/minecraft/stats/ServerStatsCounter;
private locateStatsFile(Lcom/mojang/authlib/GameProfile;)Ljava/nio/file/Path;
public getPlayerAdvancements(Lnet/minecraft/server/level/ServerPlayer;)Lnet/minecraft/server/PlayerAdvancements;
public setViewDistance(I)V
public setSimulationDistance(I)V
public getPlayers()Ljava/util/List;
public getPlayersByUUID()Ljava/util/Map;
public getPlayer(Ljava/util/UUID;)Lnet/minecraft/server/level/ServerPlayer;
public getPlayer(Ljava/lang/String;)Lnet/minecraft/server/level/ServerPlayer;
public canBypassPlayerLimit(Lnet/minecraft/server/players/NameAndId;)Z
public reloadResources()V
private synthetic lambda$getPlayerStats$0(Lcom/mojang/authlib/GameProfile;Ljava/util/UUID;)Lnet/minecraft/stats/ServerStatsCounter;
private static synthetic lambda$broadcastSystemMessage$0(Lnet/minecraft/network/chat/Component;Lnet/minecraft/server/level/ServerPlayer;)Lnet/minecraft/network/chat/Component;
private static synthetic lambda$remove$0(Lnet/minecraft/world/entity/Entity;)V
static <clinit>()V
```
