---
type: "interface"
fqcn: "net.minecraft.gametest.framework.JUnitLikeTestReporter"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.gametest.framework.JUnitLikeTestReporter

System: [[20-Systems/net.minecraft.gametest.framework|net.minecraft.gametest.framework]]

`class` public; extends `java/lang/Object`; implements `net/minecraft/gametest/framework/TestReporter`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Ljava/io/File;)V` | exact | invokespecial@2 in `SavingXmlTestReporter.<init>` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |
| calls | `save` | `(Ljava/io/File;)V` | exact | invokespecial@34 in `SavingXmlTestReporter.save` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |

## Declared members (4 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final document : Lorg/w3c/dom/Document;
private final testSuite : Lorg/w3c/dom/Element;
private final stopwatch : Lcom/google/common/base/Stopwatch;
private final destination : Ljava/io/File;
public <init>(Ljava/io/File;)V
private createTestCase(Lnet/minecraft/gametest/framework/GameTestInfo;Ljava/lang/String;)Lorg/w3c/dom/Element;
public onTestFailed(Lnet/minecraft/gametest/framework/GameTestInfo;)V
public onTestSuccess(Lnet/minecraft/gametest/framework/GameTestInfo;)V
public finish()V
public save(Ljava/io/File;)V
```
