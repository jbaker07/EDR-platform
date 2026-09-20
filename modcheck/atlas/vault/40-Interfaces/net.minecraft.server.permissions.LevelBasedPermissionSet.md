---
type: "interface"
fqcn: "net.minecraft.server.permissions.LevelBasedPermissionSet"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.permissions.LevelBasedPermissionSet

System: [[20-Systems/net.minecraft.server.permissions|net.minecraft.server.permissions]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `level()Lnet/minecraft/server/permissions/PermissionLevel;` | `` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |

## Declared members (11, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.server.permissions.LevelBasedPermissionSet extends net.minecraft.server.permissions.PermissionSet {
    public static final net.minecraft.server.permissions.LevelBasedPermissionSet ALL;
    public static final net.minecraft.server.permissions.LevelBasedPermissionSet MODERATOR;
    public static final net.minecraft.server.permissions.LevelBasedPermissionSet GAMEMASTER;
    public static final net.minecraft.server.permissions.LevelBasedPermissionSet ADMIN;
    public static final net.minecraft.server.permissions.LevelBasedPermissionSet OWNER;
    public abstract net.minecraft.server.permissions.PermissionLevel level();
    public default boolean hasPermission(net.minecraft.server.permissions.Permission);
    public default net.minecraft.server.permissions.PermissionSet union(net.minecraft.server.permissions.PermissionSet);
    public static net.minecraft.server.permissions.LevelBasedPermissionSet forLevel(net.minecraft.server.permissions.PermissionLevel);
    private static net.minecraft.server.permissions.LevelBasedPermissionSet create(net.minecraft.server.permissions.PermissionLevel);
    static {};
}
```
