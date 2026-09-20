---
type: "interface"
fqcn: "net.minecraft.gametest.framework.GlobalTestReporter"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.gametest.framework.GlobalTestReporter

System: [[20-Systems/net.minecraft.gametest.framework|net.minecraft.gametest.framework]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `replaceWith` | `(Lnet/minecraft/gametest/framework/TestReporter;)V` | exact | invokestatic@25 in `FabricGameTestRunner.runHeadlessServer` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |

## Declared members (1 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static DELEGATE : Lnet/minecraft/gametest/framework/TestReporter;
public <init>()V
public static replaceWith(Lnet/minecraft/gametest/framework/TestReporter;)V
public static onTestFailed(Lnet/minecraft/gametest/framework/GameTestInfo;)V
public static onTestSuccess(Lnet/minecraft/gametest/framework/GameTestInfo;)V
public static finish()V
static <clinit>()V
```
