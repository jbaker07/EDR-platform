---
type: "interface"
fqcn: "net.minecraft.client.multiplayer.resolver.ServerAddress"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.multiplayer.resolver.ServerAddress

System: [[20-Systems/net.minecraft.client.multiplayer|net.minecraft.client.multiplayer]]

`class` public final; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `parseString` | `(Ljava/lang/String;)Lnet/minecraft/client/multiplayer/resolver/ServerA` | exact | invokestatic@29 in `TestDedicatedServerContextImpl.lambda$connect$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (3 fields, 11 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private final hostAndPort : Lcom/google/common/net/HostAndPort;
private static final INVALID : Lnet/minecraft/client/multiplayer/resolver/ServerAddress;
public <init>(Ljava/lang/String;I)V
private <init>(Lcom/google/common/net/HostAndPort;)V
public getHost()Ljava/lang/String;
public getPort()I
public static parseString(Ljava/lang/String;)Lnet/minecraft/client/multiplayer/resolver/ServerAddress;
public static isValidAddress(Ljava/lang/String;)Z
public static parsePort(Ljava/lang/String;)I
public toString()Ljava/lang/String;
public equals(Ljava/lang/Object;)Z
public hashCode()I
static <clinit>()V
```
