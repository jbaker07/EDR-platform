---
type: "interface"
fqcn: "net.minecraft.server.players.UserWhiteListEntry"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.players.UserWhiteListEntry

System: [[20-Systems/net.minecraft.server.players|net.minecraft.server.players]]

`class` public; extends `net/minecraft/server/players/StoredUserEntry`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/server/players/NameAndId;)V` | exact | invokespecial@19 in `DedicatedServerImplUtil.lambda$whitelistClient$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (0 fields, 3 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public <init>(Lnet/minecraft/server/players/NameAndId;)V
public <init>(Lcom/google/gson/JsonObject;)V
protected serialize(Lcom/google/gson/JsonObject;)V
```
