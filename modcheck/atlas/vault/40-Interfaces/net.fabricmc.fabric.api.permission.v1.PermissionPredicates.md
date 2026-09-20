---
type: "interface"
fqcn: "net.fabricmc.fabric.api.permission.v1.PermissionPredicates"
module: "fabric-permission-api-v1"
sha256: "a3a82771b3fd9f2eb36e1098c5f90759ac877982b9ac297831a1fce9b2185bf8"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.permission.v1.PermissionPredicates

Module: [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] -- kind: class

```java
public static <T extends net.fabricmc.fabric.api.permission.v1.PermissionContextOwner> java.util.function.Predicate<T> require(net.minecraft.resources.Identifier)
public static <T extends net.fabricmc.fabric.api.permission.v1.PermissionContextOwner> java.util.function.Predicate<T> require(net.minecraft.resources.Identifier, boolean)
public static <T extends net.fabricmc.fabric.api.permission.v1.PermissionContextOwner> java.util.function.Predicate<T> require(net.minecraft.resources.Identifier, net.minecraft.server.permissions.PermissionLevel)
public static <T extends net.fabricmc.fabric.api.permission.v1.PermissionContextOwner> java.util.function.Predicate<T> require(net.fabricmc.fabric.api.permission.v1.PermissionNode<java.lang.Boolean>)
public static <T extends net.fabricmc.fabric.api.permission.v1.PermissionContextOwner> java.util.function.Predicate<T> require(net.fabricmc.fabric.api.permission.v1.PermissionNode<java.lang.Boolean>, boolean)
public static <T extends net.fabricmc.fabric.api.permission.v1.PermissionContextOwner> java.util.function.Predicate<T> require(net.fabricmc.fabric.api.permission.v1.PermissionNode<java.lang.Boolean>, net.minecraft.server.permissions.PermissionLevel)
```
