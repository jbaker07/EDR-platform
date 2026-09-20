---
type: "interface"
fqcn: "net.minecraft.gametest.framework.GlobalTestReporter"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.gametest.framework.GlobalTestReporter

System: [[20-Systems/net.minecraft.gametest.framework|net.minecraft.gametest.framework]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `replaceWith(Lnet/minecraft/gametest/framework/TestReporter;)V` | `` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |

## Declared members (7, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.gametest.framework.GlobalTestReporter {
    private static net.minecraft.gametest.framework.TestReporter DELEGATE;
    public net.minecraft.gametest.framework.GlobalTestReporter();
    public static void replaceWith(net.minecraft.gametest.framework.TestReporter);
    public static void onTestFailed(net.minecraft.gametest.framework.GameTestInfo);
    public static void onTestSuccess(net.minecraft.gametest.framework.GameTestInfo);
    public static void finish();
    static {};
}
```
