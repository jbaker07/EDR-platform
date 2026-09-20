---
type: "interface"
fqcn: "net.minecraft.server.permissions.LevelBasedPermissionSet"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.permissions.LevelBasedPermissionSet

System: [[20-Systems/net.minecraft.server.permissions|net.minecraft.server.permissions]]

`interface` public abstract; extends `java/lang/Object`; implements `net/minecraft/server/permissions/PermissionSet`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `level` | `()Lnet/minecraft/server/permissions/PermissionLevel;` | exact | invokeinterface@28 in `PermissionContext.offlinePlayer` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| calls | `level` | `()Lnet/minecraft/server/permissions/PermissionLevel;` | exact | invokeinterface@19 in `PermissionContext.offlinePlayer` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| calls | `level` | `()Lnet/minecraft/server/permissions/PermissionLevel;` | exact | invokeinterface@13 in `CommandPermissionContext.extractPermissionLevel` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |

## Declared members (5 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final ALL : Lnet/minecraft/server/permissions/LevelBasedPermissionSet;
public static final MODERATOR : Lnet/minecraft/server/permissions/LevelBasedPermissionSet;
public static final GAMEMASTER : Lnet/minecraft/server/permissions/LevelBasedPermissionSet;
public static final ADMIN : Lnet/minecraft/server/permissions/LevelBasedPermissionSet;
public static final OWNER : Lnet/minecraft/server/permissions/LevelBasedPermissionSet;
public abstract level()Lnet/minecraft/server/permissions/PermissionLevel;
public hasPermission(Lnet/minecraft/server/permissions/Permission;)Z
public union(Lnet/minecraft/server/permissions/PermissionSet;)Lnet/minecraft/server/permissions/PermissionSet;
public static forLevel(Lnet/minecraft/server/permissions/PermissionLevel;)Lnet/minecraft/server/permissions/LevelBasedPermissionSet;
private static create(Lnet/minecraft/server/permissions/PermissionLevel;)Lnet/minecraft/server/permissions/LevelBasedPermissionSet;
static <clinit>()V
```
