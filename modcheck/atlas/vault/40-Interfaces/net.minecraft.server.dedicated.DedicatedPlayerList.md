---
type: "interface"
fqcn: "net.minecraft.server.dedicated.DedicatedPlayerList"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.dedicated.DedicatedPlayerList

System: [[20-Systems/net.minecraft.server.dedicated|net.minecraft.server.dedicated]]

`class` public; extends `net/minecraft/server/players/PlayerList`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getWhiteList` | `()Lnet/minecraft/server/players/UserWhiteList;` | inherited_exact | invokevirtual@4 in `DedicatedServerImplUtil.lambda$whitelistClient$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (1 fields, 15 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
public <init>(Lnet/minecraft/server/dedicated/DedicatedServer;Lnet/minecraft/core/LayeredRegistryAccess;Lnet/minecraft/world/level/storage/PlayerDataStorage;)V
public reloadWhiteList()V
private saveIpBanList()V
private saveUserBanList()V
private loadIpBanList()V
private loadUserBanList()V
private loadOps()V
private saveOps()V
private loadWhiteList()V
private saveWhiteList()V
public isWhiteListed(Lnet/minecraft/server/players/NameAndId;)Z
public getServer()Lnet/minecraft/server/dedicated/DedicatedServer;
public canBypassPlayerLimit(Lnet/minecraft/server/players/NameAndId;)Z
public synthetic getServer()Lnet/minecraft/server/MinecraftServer;
static <clinit>()V
```
