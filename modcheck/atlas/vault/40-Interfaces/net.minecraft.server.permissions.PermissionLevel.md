---
type: "interface"
fqcn: "net.minecraft.server.permissions.PermissionLevel"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.permissions.PermissionLevel

System: [[20-Systems/net.minecraft.server.permissions|net.minecraft.server.permissions]]

`enum` public final; extends `java/lang/Enum`; implements `net/minecraft/util/StringRepresentable`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `isEqualOrHigherThan` | `(Lnet/minecraft/server/permissions/PermissionLevel;)Z` | exact | invokevirtual@19 in `PermissionContextOwner.checkPermission` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| calls | `isEqualOrHigherThan` | `(Lnet/minecraft/server/permissions/PermissionLevel;)Z` | exact | invokevirtual@14 in `PermissionPredicates.lambda$require$5` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| reads | `ADMINS` | `Lnet/minecraft/server/permissions/PermissionLevel;` | exact | getstatic@18 in `CommandPermissionContext.<clinit>` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| reads | `ALL` | `Lnet/minecraft/server/permissions/PermissionLevel;` | exact | getstatic@37 in `CommandPermissionContext.extractPermissionLevel` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| reads | `ALL` | `Lnet/minecraft/server/permissions/PermissionLevel;` | exact | getstatic@41 in `CommandPermissionContext.extractPermissionLevel` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| reads | `ALL` | `Lnet/minecraft/server/permissions/PermissionLevel;` | exact | getstatic@27 in `EntityPermissionContext.permissionLevel` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| reads | `GAMEMASTERS` | `Lnet/minecraft/server/permissions/PermissionLevel;` | exact | getstatic@9 in `CommandPermissionContext.<clinit>` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| reads | `MODERATORS` | `Lnet/minecraft/server/permissions/PermissionLevel;` | exact | getstatic@0 in `CommandPermissionContext.<clinit>` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| reads | `OWNERS` | `Lnet/minecraft/server/permissions/PermissionLevel;` | exact | getstatic@26 in `CommandPermissionContext.extractPermissionLevel` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |
| reads | `OWNERS` | `Lnet/minecraft/server/permissions/PermissionLevel;` | exact | getstatic@27 in `CommandPermissionContext.<clinit>` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |

## Declared members (11 fields, 11 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final ALL : Lnet/minecraft/server/permissions/PermissionLevel;
public static final MODERATORS : Lnet/minecraft/server/permissions/PermissionLevel;
public static final GAMEMASTERS : Lnet/minecraft/server/permissions/PermissionLevel;
public static final ADMINS : Lnet/minecraft/server/permissions/PermissionLevel;
public static final OWNERS : Lnet/minecraft/server/permissions/PermissionLevel;
public static final CODEC : Lcom/mojang/serialization/Codec;
private static final BY_ID : Ljava/util/function/IntFunction;
public static final INT_CODEC : Lcom/mojang/serialization/Codec;
private final name : Ljava/lang/String;
private final id : I
private static final synthetic $VALUES : [Lnet/minecraft/server/permissions/PermissionLevel;
public static values()[Lnet/minecraft/server/permissions/PermissionLevel;
public static valueOf(Ljava/lang/String;)Lnet/minecraft/server/permissions/PermissionLevel;
private <init>(Ljava/lang/String;ILjava/lang/String;I)V
public isEqualOrHigherThan(Lnet/minecraft/server/permissions/PermissionLevel;)Z
public static byId(I)Lnet/minecraft/server/permissions/PermissionLevel;
public id()I
public getSerializedName()Ljava/lang/String;
private static synthetic $values()[Lnet/minecraft/server/permissions/PermissionLevel;
private static synthetic lambda$static$1(Lnet/minecraft/server/permissions/PermissionLevel;)Ljava/lang/Integer;
private static synthetic lambda$static$0(Lnet/minecraft/server/permissions/PermissionLevel;)I
static <clinit>()V
```
