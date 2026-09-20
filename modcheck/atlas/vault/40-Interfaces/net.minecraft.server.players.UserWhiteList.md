---
type: "interface"
fqcn: "net.minecraft.server.players.UserWhiteList"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.players.UserWhiteList

System: [[20-Systems/net.minecraft.server.players|net.minecraft.server.players]]

`class` public; extends `net/minecraft/server/players/StoredUserList`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `add` | `(Lnet/minecraft/server/players/UserWhiteListEntry;)Z` | exact | invokevirtual@22 in `DedicatedServerImplUtil.lambda$whitelistClient$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (0 fields, 12 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public <init>(Ljava/io/File;Lnet/minecraft/server/notifications/NotificationService;)V
protected createEntry(Lcom/google/gson/JsonObject;)Lnet/minecraft/server/players/StoredUserEntry;
public isWhiteListed(Lnet/minecraft/server/players/NameAndId;)Z
public add(Lnet/minecraft/server/players/UserWhiteListEntry;)Z
public remove(Lnet/minecraft/server/players/NameAndId;)Z
public clear()V
public getUserList()[Ljava/lang/String;
protected getKeyForUser(Lnet/minecraft/server/players/NameAndId;)Ljava/lang/String;
protected synthetic getKeyForUser(Ljava/lang/Object;)Ljava/lang/String;
public synthetic remove(Ljava/lang/Object;)Z
public synthetic add(Lnet/minecraft/server/players/StoredUserEntry;)Z
private static synthetic lambda$getUserList$0(I)[Ljava/lang/String;
```
