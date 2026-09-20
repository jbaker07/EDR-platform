---
type: "interface"
fqcn: "net.minecraft.server.dedicated.ServerWatchdog"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.dedicated.ServerWatchdog

System: [[20-Systems/net.minecraft.server.dedicated|net.minecraft.server.dedicated]]

`class` public; extends `java/lang/Object`; implements `java/lang/Runnable`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `createWatchdogCrashReport` | `(Ljava/lang/String;J)Lnet/minecraft/CrashReport;` | exact | @ModifyArg at ['INVOKE'] | both | [[30-Mechanisms/fabric-crash-report-info-v1|fabric-crash-report-info-v1]] | direct_reference |

## Declared members (6 fields, 8 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
private static final MAX_SHUTDOWN_TIME : J
private static final SHUTDOWN_STATUS : I
private static final THREAD_INFO_COMPARATOR : Ljava/util/Comparator;
private final server : Lnet/minecraft/server/dedicated/DedicatedServer;
private final maxTickTimeNanos : J
public <init>(Lnet/minecraft/server/dedicated/DedicatedServer;)V
public run()V
public static createWatchdogCrashReport(Ljava/lang/String;J)Lnet/minecraft/CrashReport;
private exit()V
private synthetic lambda$run$1()Ljava/lang/String;
private static synthetic lambda$run$2(Lnet/minecraft/server/level/ServerLevel;)Ljava/lang/String;
private synthetic lambda$run$0()Ljava/lang/String;
static <clinit>()V
```
