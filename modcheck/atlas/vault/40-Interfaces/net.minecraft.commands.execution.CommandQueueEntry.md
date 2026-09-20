---
type: "interface"
fqcn: "net.minecraft.commands.execution.CommandQueueEntry"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.commands.execution.CommandQueueEntry

System: [[20-Systems/net.minecraft.commands.execution|net.minecraft.commands.execution]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `action` | `Lnet/minecraft/commands/execution/EntryAction;` | exact | @Shadow declaration | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | declared |
| wraps | `execute` | `(Lnet/minecraft/commands/execution/ExecutionContext;)V` | name_only | @WrapMethod | both | [[30-Mechanisms/fabric-entity-events-v1|fabric-entity-events-v1]] | direct_reference |

## Declared members (2 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final frame : Lnet/minecraft/commands/execution/Frame;
private final action : Lnet/minecraft/commands/execution/EntryAction;
public <init>(Lnet/minecraft/commands/execution/Frame;Lnet/minecraft/commands/execution/EntryAction;)V
public execute(Lnet/minecraft/commands/execution/ExecutionContext;)V
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public frame()Lnet/minecraft/commands/execution/Frame;
public action()Lnet/minecraft/commands/execution/EntryAction;
```
