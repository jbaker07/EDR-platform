---
type: "interface"
fqcn: "net.minecraft.gametest.framework.FunctionGameTestInstance"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.gametest.framework.FunctionGameTestInstance

System: [[20-Systems/net.minecraft.gametest.framework|net.minecraft.gametest.framework]]

`class` public; extends `net/minecraft/gametest/framework/GameTestInstance`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/gametest/framewor` | exact | invokespecial@19 in `TestAnnotationLocator$TestMethod.testInstance` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |

## Declared members (2 fields, 9 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final CODEC : Lcom/mojang/serialization/MapCodec;
private final function : Lnet/minecraft/resources/ResourceKey;
public <init>(Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/gametest/framework/TestData;)V
public run(Lnet/minecraft/gametest/framework/GameTestHelper;)V
private function()Lnet/minecraft/resources/ResourceKey;
public codec()Lcom/mojang/serialization/MapCodec;
protected typeDescription()Lnet/minecraft/network/chat/MutableComponent;
public describe()Lnet/minecraft/network/chat/Component;
private synthetic lambda$run$0()Ljava/lang/IllegalStateException;
private static synthetic lambda$static$0(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
static <clinit>()V
```
