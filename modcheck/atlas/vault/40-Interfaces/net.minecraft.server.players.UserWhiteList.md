---
type: "interface"
fqcn: "net.minecraft.server.players.UserWhiteList"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.players.UserWhiteList

System: [[20-Systems/net.minecraft.server.players|net.minecraft.server.players]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `add(Lnet/minecraft/server/players/UserWhiteListEntry;)Z` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (12, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.server.players.UserWhiteList extends net.minecraft.server.players.StoredUserList<net.minecraft.server.players.NameAndId, net.minecraft.server.players.UserWhiteListEntry> {
    public net.minecraft.server.players.UserWhiteList(java.io.File, net.minecraft.server.notifications.NotificationService);
    protected net.minecraft.server.players.StoredUserEntry<net.minecraft.server.players.NameAndId> createEntry(com.google.gson.JsonObject);
    public boolean isWhiteListed(net.minecraft.server.players.NameAndId);
    public boolean add(net.minecraft.server.players.UserWhiteListEntry);
    public boolean remove(net.minecraft.server.players.NameAndId);
    public void clear();
    public java.lang.String[] getUserList();
    protected java.lang.String getKeyForUser(net.minecraft.server.players.NameAndId);
    protected java.lang.String getKeyForUser(java.lang.Object);
    public boolean remove(java.lang.Object);
    public boolean add(net.minecraft.server.players.StoredUserEntry);
    private static java.lang.String[] lambda$getUserList$0(int);
}
```
