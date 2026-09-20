---
type: "interface"
fqcn: "net.minecraft.server.players.NameAndId"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.players.NameAndId

System: [[20-Systems/net.minecraft.server.players|net.minecraft.server.players]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lcom/mojang/authlib/GameProfile;)V` | exact | invokespecial@16 in `DedicatedServerImplUtil.lambda$whitelistClient$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `<init>` | `(Lcom/mojang/authlib/GameProfile;)V` | exact | invokespecial@15 in `RegistrySyncManager.configureClient` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `<init>` | `(Ljava/util/UUID;Ljava/lang/String;)V` | exact | invokespecial@22 in `PermissionContext.offlinePlayer` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| calls | `id` | `()Ljava/util/UUID;` | exact | invokevirtual@30 in `PermissionContext.offlinePlayer` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| calls | `name` | `()Ljava/lang/String;` | exact | invokevirtual@46 in `PermissionContext.offlinePlayer` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |

## Declared members (3 fields, 13 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final id : Ljava/util/UUID;
private final name : Ljava/lang/String;
public static final CODEC : Lcom/mojang/serialization/Codec;
public <init>(Lcom/mojang/authlib/GameProfile;)V
public <init>(Lcom/mojang/authlib/services/response/NameAndId;)V
public <init>(Ljava/util/UUID;Ljava/lang/String;)V
public static fromJson(Lcom/google/gson/JsonObject;)Lnet/minecraft/server/players/NameAndId;
public appendTo(Lcom/google/gson/JsonObject;)V
public static createOffline(Ljava/lang/String;)Lnet/minecraft/server/players/NameAndId;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public id()Ljava/util/UUID;
public name()Ljava/lang/String;
private static synthetic lambda$static$0(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
static <clinit>()V
```
