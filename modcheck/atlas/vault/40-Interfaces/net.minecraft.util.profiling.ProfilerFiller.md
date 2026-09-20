---
type: "interface"
fqcn: "net.minecraft.util.profiling.ProfilerFiller"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.profiling.ProfilerFiller

System: [[20-Systems/net.minecraft.util.profiling|net.minecraft.util.profiling]]

`interface` public abstract; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `pop` | `()V` | exact | invokeinterface@40 in `ClientCommandInternals.executeCommand` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `pop` | `()V` | exact | invokeinterface@61 in `ClientCommandInternals.executeCommand` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `pop` | `()V` | exact | invokeinterface@101 in `ClientCommandInternals.executeCommand` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `pop` | `()V` | exact | invokeinterface@137 in `ClientCommandInternals.executeCommand` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `pop` | `()V` | exact | invokeinterface@177 in `ClientCommandInternals.executeCommand` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `pop` | `()V` | exact | invokeinterface@190 in `ClientCommandInternals.executeCommand` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `push` | `(Ljava/lang/String;)V` | exact | invokeinterface@4 in `ClientCommandInternals.executeCommand` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |

## Declared members (1 fields, 18 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final ROOT : Ljava/lang/String;
public abstract startTick()V
public abstract endTick()V
public abstract push(Ljava/lang/String;)V
public abstract push(Ljava/util/function/Supplier;)V
public abstract pop()V
public abstract popPush(Ljava/lang/String;)V
public abstract popPush(Ljava/util/function/Supplier;)V
public addZoneText(Ljava/lang/String;)V
public addZoneValue(J)V
public setZoneColor(I)V
public zone(Ljava/lang/String;)Lnet/minecraft/util/profiling/Zone;
public zone(Ljava/util/function/Supplier;)Lnet/minecraft/util/profiling/Zone;
public abstract markForCharting(Lnet/minecraft/util/profiling/metrics/MetricCategory;)V
public incrementCounter(Ljava/lang/String;)V
public abstract incrementCounter(Ljava/lang/String;I)V
public incrementCounter(Ljava/util/function/Supplier;)V
public abstract incrementCounter(Ljava/util/function/Supplier;I)V
public static combine(Lnet/minecraft/util/profiling/ProfilerFiller;Lnet/minecraft/util/profiling/ProfilerFiller;)Lnet/minecraft/util/profiling/ProfilerFiller;
```
