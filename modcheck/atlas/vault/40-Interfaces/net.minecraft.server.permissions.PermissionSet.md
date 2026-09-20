---
type: "interface"
fqcn: "net.minecraft.server.permissions.PermissionSet"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.permissions.PermissionSet

System: [[20-Systems/net.minecraft.server.permissions|net.minecraft.server.permissions]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `hasPermission(Lnet/minecraft/server/permissions/Permission;)Z` | `` | unknown | [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] | direct_reference |

## Declared members (7, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.server.permissions.PermissionSet {
    public static final net.minecraft.server.permissions.PermissionSet NO_PERMISSIONS;
    public static final net.minecraft.server.permissions.PermissionSet ALL_PERMISSIONS;
    public abstract boolean hasPermission(net.minecraft.server.permissions.Permission);
    public default net.minecraft.server.permissions.PermissionSet union(net.minecraft.server.permissions.PermissionSet);
    private static boolean lambda$static$1(net.minecraft.server.permissions.Permission);
    private static boolean lambda$static$0(net.minecraft.server.permissions.Permission);
    static {};
}
```
