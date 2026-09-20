---
type: "interface"
fqcn: "net.minecraft.server.permissions.Permissions"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.permissions.Permissions

System: [[20-Systems/net.minecraft.server.permissions|net.minecraft.server.permissions]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `COMMANDS_ADMIN` | `Lnet/minecraft/server/permissions/Permission;` | exact | getstatic@21 in `CommandPermissionContext.<clinit>` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| reads | `COMMANDS_GAMEMASTER` | `Lnet/minecraft/server/permissions/Permission;` | exact | getstatic@12 in `CommandPermissionContext.<clinit>` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| reads | `COMMANDS_MODERATOR` | `Lnet/minecraft/server/permissions/Permission;` | exact | getstatic@3 in `CommandPermissionContext.<clinit>` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| reads | `COMMANDS_OWNER` | `Lnet/minecraft/server/permissions/Permission;` | exact | getstatic@30 in `CommandPermissionContext.<clinit>` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |

## Declared members (10 fields, 2 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final COMMANDS_MODERATOR : Lnet/minecraft/server/permissions/Permission;
public static final COMMANDS_GAMEMASTER : Lnet/minecraft/server/permissions/Permission;
public static final COMMANDS_ADMIN : Lnet/minecraft/server/permissions/Permission;
public static final COMMANDS_OWNER : Lnet/minecraft/server/permissions/Permission;
public static final COMMANDS_ENTITY_SELECTORS : Lnet/minecraft/server/permissions/Permission;
public static final CHAT_SEND_MESSAGES : Lnet/minecraft/server/permissions/Permission;
public static final CHAT_SEND_COMMANDS : Lnet/minecraft/server/permissions/Permission;
public static final CHAT_RECEIVE_PLAYER_MESSAGES : Lnet/minecraft/server/permissions/Permission;
public static final CHAT_RECEIVE_SYSTEM_MESSAGES : Lnet/minecraft/server/permissions/Permission;
public static final CHAT_PERMISSIONS : Ljava/util/Set;
public <init>()V
static <clinit>()V
```
