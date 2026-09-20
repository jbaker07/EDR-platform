---
type: "interface"
fqcn: "net.minecraft.server.notifications.NotificationManager"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.notifications.NotificationManager

System: [[20-Systems/net.minecraft.server.notifications|net.minecraft.server.notifications]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/server/notifications/NotificationService`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `serverStarted` | `()V` | exact | invokevirtual@8 in `DedicatedServerMixin.afterServerStartedEvent` | unknown | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |

## Declared members (2 fields, 40 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final notificationServices : Ljava/util/List;
private server : Lnet/minecraft/server/dedicated/DedicatedServer;
public <init>()V
public registerService(Lnet/minecraft/server/notifications/NotificationService;)V
public setServer(Lnet/minecraft/server/dedicated/DedicatedServer;)V
public server()Lnet/minecraft/server/dedicated/DedicatedServer;
public playerJoined(Lnet/minecraft/server/level/ServerPlayer;)V
public playerLeft(Lnet/minecraft/server/level/ServerPlayer;)V
public serverStarted()V
public serverShuttingDown()V
public serverSaveStarted()V
public serverSaveCompleted()V
public serverActivityOccured()V
public worldUpgradeStarted()V
public worldUpgradeProgress(F)V
public worldUpgradeFinished()V
public worldUpgradeFailed(Ljava/lang/String;)V
public playerOped(Lnet/minecraft/server/players/ServerOpListEntry;)V
public playerDeoped(Lnet/minecraft/server/players/ServerOpListEntry;)V
public playerAddedToAllowlist(Lnet/minecraft/server/players/NameAndId;)V
public playerRemovedFromAllowlist(Lnet/minecraft/server/players/NameAndId;)V
public ipBanned(Lnet/minecraft/server/players/IpBanListEntry;)V
public ipUnbanned(Ljava/lang/String;)V
public playerBanned(Lnet/minecraft/server/players/UserBanListEntry;)V
public playerUnbanned(Lnet/minecraft/server/players/NameAndId;)V
public onGameRuleChanged(Lnet/minecraft/world/level/gamerules/GameRule;Ljava/lang/Object;)V
public statusHeartbeat()V
private static synthetic lambda$onGameRuleChanged$0(Lnet/minecraft/world/level/gamerules/GameRule;Ljava/lang/Object;Lnet/minecraft/server/notifications/NotificationService;)V
private static synthetic lambda$playerUnbanned$0(Lnet/minecraft/server/players/NameAndId;Lnet/minecraft/server/notifications/NotificationService;)V
private static synthetic lambda$playerBanned$0(Lnet/minecraft/server/players/UserBanListEntry;Lnet/minecraft/server/notifications/NotificationService;)V
private static synthetic lambda$ipUnbanned$0(Ljava/lang/String;Lnet/minecraft/server/notifications/NotificationService;)V
private static synthetic lambda$ipBanned$0(Lnet/minecraft/server/players/IpBanListEntry;Lnet/minecraft/server/notifications/NotificationService;)V
private static synthetic lambda$playerRemovedFromAllowlist$0(Lnet/minecraft/server/players/NameAndId;Lnet/minecraft/server/notifications/NotificationService;)V
private static synthetic lambda$playerAddedToAllowlist$0(Lnet/minecraft/server/players/NameAndId;Lnet/minecraft/server/notifications/NotificationService;)V
private static synthetic lambda$playerDeoped$0(Lnet/minecraft/server/players/ServerOpListEntry;Lnet/minecraft/server/notifications/NotificationService;)V
private static synthetic lambda$playerOped$0(Lnet/minecraft/server/players/ServerOpListEntry;Lnet/minecraft/server/notifications/NotificationService;)V
private static synthetic lambda$worldUpgradeFailed$0(Ljava/lang/String;Lnet/minecraft/server/notifications/NotificationService;)V
private static synthetic lambda$worldUpgradeFinished$0(Lnet/minecraft/server/notifications/NotificationService;)V
private static synthetic lambda$worldUpgradeProgress$0(FLnet/minecraft/server/notifications/NotificationService;)V
private static synthetic lambda$worldUpgradeStarted$0(Lnet/minecraft/server/notifications/NotificationService;)V
private static synthetic lambda$playerLeft$0(Lnet/minecraft/server/level/ServerPlayer;Lnet/minecraft/server/notifications/NotificationService;)V
private static synthetic lambda$playerJoined$0(Lnet/minecraft/server/level/ServerPlayer;Lnet/minecraft/server/notifications/NotificationService;)V
```
