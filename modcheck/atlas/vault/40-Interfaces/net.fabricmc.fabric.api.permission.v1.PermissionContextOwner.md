---
type: "interface"
fqcn: "net.fabricmc.fabric.api.permission.v1.PermissionContextOwner"
module: "fabric-permission-api-v1"
sha256: "a3a82771b3fd9f2eb36e1098c5f90759ac877982b9ac297831a1fce9b2185bf8"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.permission.v1.PermissionContextOwner

Module: [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] -- kind: interface

```java
public net.fabricmc.fabric.api.permission.v1.PermissionContext getPermissionContext()
public net.fabricmc.fabric.api.util.TriState checkPermission(net.minecraft.resources.Identifier)
public boolean checkPermission(net.minecraft.resources.Identifier, boolean)
public boolean checkPermission(net.minecraft.resources.Identifier, net.minecraft.server.permissions.PermissionLevel)
public java.lang.Object checkPermission(net.fabricmc.fabric.api.permission.v1.PermissionNode)
public java.lang.Object checkPermission(net.fabricmc.fabric.api.permission.v1.PermissionNode, java.lang.Object)
```
