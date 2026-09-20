---
type: "interface"
fqcn: "net.minecraft.util.profiling.Profiler"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.profiling.Profiler

System: [[20-Systems/net.minecraft.util.profiling|net.minecraft.util.profiling]]

`class` public final; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `get` | `()Lnet/minecraft/util/profiling/ProfilerFiller;` | exact | invokestatic@0 in `ClientCommandInternals.executeCommand` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `get` | `()Lnet/minecraft/util/profiling/ProfilerFiller;` | exact | invokestatic@37 in `ClientCommandInternals.executeCommand` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `get` | `()Lnet/minecraft/util/profiling/ProfilerFiller;` | exact | invokestatic@58 in `ClientCommandInternals.executeCommand` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `get` | `()Lnet/minecraft/util/profiling/ProfilerFiller;` | exact | invokestatic@98 in `ClientCommandInternals.executeCommand` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `get` | `()Lnet/minecraft/util/profiling/ProfilerFiller;` | exact | invokestatic@134 in `ClientCommandInternals.executeCommand` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `get` | `()Lnet/minecraft/util/profiling/ProfilerFiller;` | exact | invokestatic@174 in `ClientCommandInternals.executeCommand` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |
| calls | `get` | `()Lnet/minecraft/util/profiling/ProfilerFiller;` | exact | invokestatic@187 in `ClientCommandInternals.executeCommand` | unknown | [[30-Mechanisms/fabric-command-api-v2|fabric-command-api-v2]] | direct_reference |

## Declared members (3 fields, 9 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final TRACY_FILLER : Ljava/lang/ThreadLocal;
private static final ACTIVE : Ljava/lang/ThreadLocal;
private static final ACTIVE_COUNT : Ljava/util/concurrent/atomic/AtomicInteger;
private <init>()V
public static use(Lnet/minecraft/util/profiling/ProfilerFiller;)Lnet/minecraft/util/profiling/Profiler$Scope;
private static startUsing(Lnet/minecraft/util/profiling/ProfilerFiller;)V
private static stopUsing()V
private static decorateFiller(Lnet/minecraft/util/profiling/ProfilerFiller;)Lnet/minecraft/util/profiling/ProfilerFiller;
public static get()Lnet/minecraft/util/profiling/ProfilerFiller;
private static getDefaultFiller()Lnet/minecraft/util/profiling/ProfilerFiller;
private static synthetic lambda$use$0()V
static <clinit>()V
```
