---
type: "interface"
fqcn: "net.minecraft.util.profiling.Profiler"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.profiling.Profiler

System: [[20-Systems/net.minecraft.util.profiling|net.minecraft.util.profiling]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `get()Lnet/minecraft/util/profiling/ProfilerFiller;` | `` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |

## Declared members (12, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.util.profiling.Profiler {
    private static final java.lang.ThreadLocal<net.minecraft.util.profiling.TracyZoneFiller> TRACY_FILLER;
    private static final java.lang.ThreadLocal<net.minecraft.util.profiling.ProfilerFiller> ACTIVE;
    private static final java.util.concurrent.atomic.AtomicInteger ACTIVE_COUNT;
    private net.minecraft.util.profiling.Profiler();
    public static net.minecraft.util.profiling.Profiler$Scope use(net.minecraft.util.profiling.ProfilerFiller);
    private static void startUsing(net.minecraft.util.profiling.ProfilerFiller);
    private static void stopUsing();
    private static net.minecraft.util.profiling.ProfilerFiller decorateFiller(net.minecraft.util.profiling.ProfilerFiller);
    public static net.minecraft.util.profiling.ProfilerFiller get();
    private static net.minecraft.util.profiling.ProfilerFiller getDefaultFiller();
    private static void lambda$use$0();
    static {};
}
```
