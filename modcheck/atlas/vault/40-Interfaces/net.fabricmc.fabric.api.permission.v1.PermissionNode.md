---
type: "interface"
fqcn: "net.fabricmc.fabric.api.permission.v1.PermissionNode"
module: "fabric-permission-api-v1"
sha256: "a3a82771b3fd9f2eb36e1098c5f90759ac877982b9ac297831a1fce9b2185bf8"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.permission.v1.PermissionNode

Module: [[30-Mechanisms/fabric-permission-api-v1|fabric-permission-api-v1]] -- kind: interface

```java
public static net.fabricmc.fabric.api.permission.v1.PermissionNode of(net.minecraft.resources.Identifier)
public static net.fabricmc.fabric.api.permission.v1.PermissionNode of(java.lang.String, java.lang.String)
public static net.fabricmc.fabric.api.permission.v1.PermissionNode ofInteger(net.minecraft.resources.Identifier)
public static net.fabricmc.fabric.api.permission.v1.PermissionNode ofInteger(java.lang.String, java.lang.String)
public static net.fabricmc.fabric.api.permission.v1.PermissionNode ofString(net.minecraft.resources.Identifier)
public static net.fabricmc.fabric.api.permission.v1.PermissionNode ofString(java.lang.String, java.lang.String)
public static net.fabricmc.fabric.api.permission.v1.PermissionNode ofCustom(net.minecraft.resources.Identifier, com.mojang.serialization.Codec, java.lang.Class)
public static net.fabricmc.fabric.api.permission.v1.PermissionNode ofCustom(java.lang.String, java.lang.String, com.mojang.serialization.Codec, java.lang.Class)
public static net.fabricmc.fabric.api.permission.v1.PermissionNode ofCustom(net.minecraft.resources.Identifier, com.mojang.serialization.Codec, java.util.function.Predicate)
public static net.fabricmc.fabric.api.permission.v1.PermissionNode ofCustom(java.lang.String, java.lang.String, com.mojang.serialization.Codec, java.util.function.Predicate)
public abstract java.lang.Object cast(java.lang.Object)
public abstract net.minecraft.resources.Identifier key()
public abstract com.mojang.serialization.Codec codec()
```
