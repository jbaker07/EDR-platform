---
type: "interface"
fqcn: "net.minecraft.server.players.UserWhiteListEntry"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.players.UserWhiteListEntry

System: [[20-Systems/net.minecraft.server.players|net.minecraft.server.players]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Lnet/minecraft/server/players/NameAndId;)V` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (3, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.server.players.UserWhiteListEntry extends net.minecraft.server.players.StoredUserEntry<net.minecraft.server.players.NameAndId> {
    public net.minecraft.server.players.UserWhiteListEntry(net.minecraft.server.players.NameAndId);
    public net.minecraft.server.players.UserWhiteListEntry(com.google.gson.JsonObject);
    protected void serialize(com.google.gson.JsonObject);
}
```
