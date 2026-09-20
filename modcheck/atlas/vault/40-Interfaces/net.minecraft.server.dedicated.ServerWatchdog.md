---
type: "interface"
fqcn: "net.minecraft.server.dedicated.ServerWatchdog"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.dedicated.ServerWatchdog

System: [[20-Systems/net.minecraft.server.dedicated|net.minecraft.server.dedicated]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `createWatchdogCrashReport(Ljava/lang/String;J)Lnet/minecraft/CrashReport;` | `@ModifyArg at INVOKE Ljava/lang/StringBuilder;append(Ljava/lang/Object;)Ljava/la` | both | [[30-Mechanisms/fabric-crash-report-info-v1|fabric-crash-report-info-v1]] | direct_reference |

## Declared members (14, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.server.dedicated.ServerWatchdog implements java.lang.Runnable {
    private static final org.slf4j.Logger LOGGER;
    private static final long MAX_SHUTDOWN_TIME;
    private static final int SHUTDOWN_STATUS;
    private static final java.util.Comparator<java.lang.management.ThreadInfo> THREAD_INFO_COMPARATOR;
    private final net.minecraft.server.dedicated.DedicatedServer server;
    private final long maxTickTimeNanos;
    public net.minecraft.server.dedicated.ServerWatchdog(net.minecraft.server.dedicated.DedicatedServer);
    public void run();
    public static net.minecraft.CrashReport createWatchdogCrashReport(java.lang.String, long);
    private void exit();
    private java.lang.String lambda$run$1() throws java.lang.Exception;
    private static java.lang.String lambda$run$2(net.minecraft.server.level.ServerLevel);
    private java.lang.String lambda$run$0() throws java.lang.Exception;
    static {};
}
```
