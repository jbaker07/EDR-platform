---
type: "interface"
fqcn: "net.minecraft.util.profiling.ProfilerFiller"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.profiling.ProfilerFiller

System: [[20-Systems/net.minecraft.util.profiling|net.minecraft.util.profiling]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `pop()V` | `` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `push(Ljava/lang/String;)V` | `` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |

## Declared members (19, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.util.profiling.ProfilerFiller {
    public static final java.lang.String ROOT;
    public abstract void startTick();
    public abstract void endTick();
    public abstract void push(java.lang.String);
    public abstract void push(java.util.function.Supplier<java.lang.String>);
    public abstract void pop();
    public abstract void popPush(java.lang.String);
    public abstract void popPush(java.util.function.Supplier<java.lang.String>);
    public default void addZoneText(java.lang.String);
    public default void addZoneValue(long);
    public default void setZoneColor(int);
    public default net.minecraft.util.profiling.Zone zone(java.lang.String);
    public default net.minecraft.util.profiling.Zone zone(java.util.function.Supplier<java.lang.String>);
    public abstract void markForCharting(net.minecraft.util.profiling.metrics.MetricCategory);
    public default void incrementCounter(java.lang.String);
    public abstract void incrementCounter(java.lang.String, int);
    public default void incrementCounter(java.util.function.Supplier<java.lang.String>);
    public abstract void incrementCounter(java.util.function.Supplier<java.lang.String>, int);
    public static net.minecraft.util.profiling.ProfilerFiller combine(net.minecraft.util.profiling.ProfilerFiller, net.minecraft.util.profiling.ProfilerFiller);
}
```
