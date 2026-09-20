---
type: "interface"
fqcn: "net.minecraft.gametest.framework.TestData"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.gametest.framework.TestData

System: [[20-Systems/net.minecraft.gametest.framework|net.minecraft.gametest.framework]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Ljava/lang/Object;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft` | exact | invokespecial@141 in `TestAnnotationLocator$TestMethod.testData` | unknown | [[30-Mechanisms/fabric-gametest-api-v1|fabric-gametest-api-v1]] | direct_reference |

## Declared members (13 fields, 21 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final environment : Ljava/lang/Object;
private final dimension : Lnet/minecraft/resources/ResourceKey;
private final structure : Lnet/minecraft/resources/Identifier;
private final maxTicks : I
private final setupTicks : I
private final required : Z
private final rotation : Lnet/minecraft/world/level/block/Rotation;
private final manualOnly : Z
private final maxAttempts : I
private final requiredSuccesses : I
private final skyAccess : Z
private final padding : I
public static final CODEC : Lcom/mojang/serialization/MapCodec;
public <init>(Ljava/lang/Object;Lnet/minecraft/resources/Identifier;IIZLnet/minecraft/world/level/block/Rotation;)V
public <init>(Ljava/lang/Object;Lnet/minecraft/resources/Identifier;IIZ)V
public <init>(Ljava/lang/Object;Lnet/minecraft/resources/ResourceKey;Lnet/minecraft/resources/Identifier;IIZLnet/minecraft/world/level/block/Rotation;ZIIZI)V
public map(Ljava/util/function/Function;)Lnet/minecraft/gametest/framework/TestData;
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public environment()Ljava/lang/Object;
public dimension()Lnet/minecraft/resources/ResourceKey;
public structure()Lnet/minecraft/resources/Identifier;
public maxTicks()I
public setupTicks()I
public required()Z
public rotation()Lnet/minecraft/world/level/block/Rotation;
public manualOnly()Z
public maxAttempts()I
public requiredSuccesses()I
public skyAccess()Z
public padding()I
private static synthetic lambda$static$0(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
static <clinit>()V
```
