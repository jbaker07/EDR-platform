---
type: "interface"
fqcn: "net.minecraft.server.permissions.PermissionSet"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.permissions.PermissionSet

System: [[20-Systems/net.minecraft.server.permissions|net.minecraft.server.permissions]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `hasPermission` | `(Lnet/minecraft/server/permissions/Permission;)Z` | exact | invokeinterface@81 in `CommandPermissionContext.extractPermissionLevel` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| reads | `ALL_PERMISSIONS` | `Lnet/minecraft/server/permissions/PermissionSet;` | exact | getstatic@20 in `CommandPermissionContext.extractPermissionLevel` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| reads | `NO_PERMISSIONS` | `Lnet/minecraft/server/permissions/PermissionSet;` | exact | getstatic@31 in `CommandPermissionContext.extractPermissionLevel` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |

## Declared members (2 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final NO_PERMISSIONS : Lnet/minecraft/server/permissions/PermissionSet;
public static final ALL_PERMISSIONS : Lnet/minecraft/server/permissions/PermissionSet;
public abstract hasPermission(Lnet/minecraft/server/permissions/Permission;)Z
public union(Lnet/minecraft/server/permissions/PermissionSet;)Lnet/minecraft/server/permissions/PermissionSet;
private static synthetic lambda$static$1(Lnet/minecraft/server/permissions/Permission;)Z
private static synthetic lambda$static$0(Lnet/minecraft/server/permissions/Permission;)Z
static <clinit>()V
```
