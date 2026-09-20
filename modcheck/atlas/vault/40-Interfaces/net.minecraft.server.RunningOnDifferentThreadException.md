---
type: "interface"
fqcn: "net.minecraft.server.RunningOnDifferentThreadException"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.RunningOnDifferentThreadException

System: [[20-Systems/net.minecraft.server|net.minecraft.server]]

`class` public final; extends `java/lang/RuntimeException`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `RUNNING_ON_DIFFERENT_THREAD` | `Lnet/minecraft/server/RunningOnDifferentThreadException;` | exact | getstatic@89 in `AbstractChanneledNetworkAddon.handle` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (1 fields, 3 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final RUNNING_ON_DIFFERENT_THREAD : Lnet/minecraft/server/RunningOnDifferentThreadException;
private <init>()V
public fillInStackTrace()Ljava/lang/Throwable;
static <clinit>()V
```
