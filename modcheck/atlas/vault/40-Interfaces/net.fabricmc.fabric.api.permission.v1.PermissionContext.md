---
type: "interface"
fqcn: "net.fabricmc.fabric.api.permission.v1.PermissionContext"
module: "fabric-permission-api-v1"
sha256: "a3a82771b3fd9f2eb36e1098c5f90759ac877982b9ac297831a1fce9b2185bf8"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.permission.v1.PermissionContext

Module: [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] -- kind: interface

```java
public static final net.fabricmc.fabric.api.permission.v1.PermissionContext$Key NAME
public static final net.fabricmc.fabric.api.permission.v1.PermissionContext$Key POSITION
public static final net.fabricmc.fabric.api.permission.v1.PermissionContext$Key BLOCK_POSITION
public static final net.fabricmc.fabric.api.permission.v1.PermissionContext$Key ENTITY
public static final net.fabricmc.fabric.api.permission.v1.PermissionContext$Key COMMAND_SOURCE_STACK
public static final net.fabricmc.fabric.api.permission.v1.PermissionContext$Key LEVEL
public static final net.fabricmc.fabric.api.permission.v1.PermissionContext$Key SERVER
public static net.fabricmc.fabric.api.permission.v1.MutablePermissionContext create(java.util.UUID, net.fabricmc.fabric.api.permission.v1.PermissionContext$Type, net.minecraft.server.permissions.PermissionLevel)
public static java.util.concurrent.CompletableFuture offlinePlayer(java.util.UUID, net.minecraft.server.MinecraftServer)
public static java.util.concurrent.CompletableFuture offlinePlayer(net.minecraft.server.players.NameAndId, net.minecraft.server.MinecraftServer)
public static net.fabricmc.fabric.api.permission.v1.PermissionContext$Key key(net.minecraft.resources.Identifier)
public abstract java.util.UUID uuid()
public abstract net.fabricmc.fabric.api.permission.v1.PermissionContext$Type type()
public abstract java.lang.Object get(net.fabricmc.fabric.api.permission.v1.PermissionContext$Key)
public java.lang.Object orElse(net.fabricmc.fabric.api.permission.v1.PermissionContext$Key, java.lang.Object)
public net.fabricmc.fabric.api.permission.v1.MutablePermissionContext mutable()
public abstract net.minecraft.server.permissions.PermissionLevel permissionLevel()
public abstract java.util.Set keys()
public net.fabricmc.fabric.api.permission.v1.PermissionContext getPermissionContext()
```
