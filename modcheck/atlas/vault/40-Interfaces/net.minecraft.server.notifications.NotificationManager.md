---
type: "interface"
fqcn: "net.minecraft.server.notifications.NotificationManager"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.notifications.NotificationManager

System: [[20-Systems/net.minecraft.server.notifications|net.minecraft.server.notifications]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `serverStarted()V` | `` | server | [[30-Mechanisms/fabric-lifecycle-events-v1|fabric-lifecycle-events-v1]] | direct_reference |

## Declared members (42, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.server.notifications.NotificationManager implements net.minecraft.server.notifications.NotificationService {
    private final java.util.List<net.minecraft.server.notifications.NotificationService> notificationServices;
    private net.minecraft.server.dedicated.DedicatedServer server;
    public net.minecraft.server.notifications.NotificationManager();
    public void registerService(net.minecraft.server.notifications.NotificationService);
    public void setServer(net.minecraft.server.dedicated.DedicatedServer);
    public net.minecraft.server.dedicated.DedicatedServer server();
    public void playerJoined(net.minecraft.server.level.ServerPlayer);
    public void playerLeft(net.minecraft.server.level.ServerPlayer);
    public void serverStarted();
    public void serverShuttingDown();
    public void serverSaveStarted();
    public void serverSaveCompleted();
    public void serverActivityOccured();
    public void worldUpgradeStarted();
    public void worldUpgradeProgress(float);
    public void worldUpgradeFinished();
    public void worldUpgradeFailed(java.lang.String);
    public void playerOped(net.minecraft.server.players.ServerOpListEntry);
    public void playerDeoped(net.minecraft.server.players.ServerOpListEntry);
    public void playerAddedToAllowlist(net.minecraft.server.players.NameAndId);
    public void playerRemovedFromAllowlist(net.minecraft.server.players.NameAndId);
    public void ipBanned(net.minecraft.server.players.IpBanListEntry);
    public void ipUnbanned(java.lang.String);
    public void playerBanned(net.minecraft.server.players.UserBanListEntry);
    public void playerUnbanned(net.minecraft.server.players.NameAndId);
    public <T> void onGameRuleChanged(net.minecraft.world.level.gamerules.GameRule<T>, T);
    public void statusHeartbeat();
    private static void lambda$onGameRuleChanged$0(net.minecraft.world.level.gamerules.GameRule, java.lang.Object, net.minecraft.server.notifications.NotificationService);
    private static void lambda$playerUnbanned$0(net.minecraft.server.players.NameAndId, net.minecraft.server.notifications.NotificationService);
    private static void lambda$playerBanned$0(net.minecraft.server.players.UserBanListEntry, net.minecraft.server.notifications.NotificationService);
    private static void lambda$ipUnbanned$0(java.lang.String, net.minecraft.server.notifications.NotificationService);
    private static void lambda$ipBanned$0(net.minecraft.server.players.IpBanListEntry, net.minecraft.server.notifications.NotificationService);
    private static void lambda$playerRemovedFromAllowlist$0(net.minecraft.server.players.NameAndId, net.minecraft.server.notifications.NotificationService);
    private static void lambda$playerAddedToAllowlist$0(net.minecraft.server.players.NameAndId, net.minecraft.server.notifications.NotificationService);
    private static void lambda$playerDeoped$0(net.minecraft.server.players.ServerOpListEntry, net.minecraft.server.notifications.NotificationService);
    private static void lambda$playerOped$0(net.minecraft.server.players.ServerOpListEntry, net.minecraft.server.notifications.NotificationService);
    private static void lambda$worldUpgradeFailed$0(java.lang.String, net.minecraft.server.notifications.NotificationService);
    private static void lambda$worldUpgradeFinished$0(net.minecraft.server.notifications.NotificationService);
    private static void lambda$worldUpgradeProgress$0(float, net.minecraft.server.notifications.NotificationService);
    private static void lambda$worldUpgradeStarted$0(net.minecraft.server.notifications.NotificationService);
    private static void lambda$playerLeft$0(net.minecraft.server.level.ServerPlayer, net.minecraft.server.notifications.NotificationService);
    private static void lambda$playerJoined$0(net.minecraft.server.level.ServerPlayer, net.minecraft.server.notifications.NotificationService);
}
```
