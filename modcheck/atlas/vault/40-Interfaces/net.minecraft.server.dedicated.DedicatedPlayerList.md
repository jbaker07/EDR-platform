---
type: "interface"
fqcn: "net.minecraft.server.dedicated.DedicatedPlayerList"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.dedicated.DedicatedPlayerList

System: [[20-Systems/net.minecraft.server.dedicated|net.minecraft.server.dedicated]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getWhiteList()Lnet/minecraft/server/players/UserWhiteList;` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (16, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.server.dedicated.DedicatedPlayerList extends net.minecraft.server.players.PlayerList {
    private static final org.slf4j.Logger LOGGER;
    public net.minecraft.server.dedicated.DedicatedPlayerList(net.minecraft.server.dedicated.DedicatedServer, net.minecraft.core.LayeredRegistryAccess<net.minecraft.server.RegistryLayer>, net.minecraft.world.level.storage.PlayerDataStorage);
    public void reloadWhiteList();
    private void saveIpBanList();
    private void saveUserBanList();
    private void loadIpBanList();
    private void loadUserBanList();
    private void loadOps();
    private void saveOps();
    private void loadWhiteList();
    private void saveWhiteList();
    public boolean isWhiteListed(net.minecraft.server.players.NameAndId);
    public net.minecraft.server.dedicated.DedicatedServer getServer();
    public boolean canBypassPlayerLimit(net.minecraft.server.players.NameAndId);
    public net.minecraft.server.MinecraftServer getServer();
    static {};
}
```
