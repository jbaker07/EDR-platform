---
type: "interface"
fqcn: "net.minecraft.util.datafix.DataFixers"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.datafix.DataFixers

System: [[20-Systems/net.minecraft.util.datafix|net.minecraft.util.datafix]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getDataFixer` | `()Lcom/mojang/datafixers/DataFixer;` | exact | invokestatic@74 in `CreateWorldScreenMixin.createLevelDataForServers` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (5 fields, 30 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final SAME : Ljava/util/function/BiFunction;
private static final SAME_NAMESPACED : Ljava/util/function/BiFunction;
private static final DATA_FIXER : Lcom/mojang/datafixers/DataFixerBuilder$Result;
private static final FILE_FIXER : Lnet/minecraft/util/filefix/FileFixerUpper;
public static final BLENDING_VERSION : I
private <init>()V
public static getDataFixer()Lcom/mojang/datafixers/DataFixer;
public static getFileFixer()Lnet/minecraft/util/filefix/FileFixerUpper;
public static optimize(Ljava/util/Set;)Ljava/util/concurrent/CompletableFuture;
private static addFixers(Lcom/mojang/datafixers/DataFixerBuilder;Lnet/minecraft/util/filefix/FileFixerUpper$Builder;)V
private static createRenamerNoNamespace(Ljava/util/Map;)Ljava/util/function/UnaryOperator;
private static createRenamer(Ljava/util/Map;)Ljava/util/function/UnaryOperator;
private static createRenamer(Ljava/lang/String;Ljava/lang/String;)Ljava/util/function/UnaryOperator;
private static synthetic lambda$createRenamer$1(Ljava/lang/String;Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;
private static synthetic lambda$createRenamer$0(Ljava/util/Map;Ljava/lang/String;)Ljava/lang/String;
private static synthetic lambda$createRenamerNoNamespace$0(Ljava/util/Map;Ljava/lang/String;)Ljava/lang/String;
private static synthetic lambda$addFixers$17(Lcom/mojang/serialization/Dynamic;)Lcom/mojang/serialization/Dynamic;
private static synthetic lambda$addFixers$16(Lcom/mojang/serialization/Dynamic;)Lcom/mojang/serialization/Dynamic;
private static synthetic lambda$addFixers$15(Ljava/lang/String;)Ljava/lang/String;
private static synthetic lambda$addFixers$14(Ljava/lang/String;)Ljava/lang/String;
private static synthetic lambda$addFixers$13(D)D
private static synthetic lambda$addFixers$12(D)D
private static synthetic lambda$addFixers$11(D)D
private static synthetic lambda$addFixers$10(D)D
private static synthetic lambda$addFixers$9(D)D
private static synthetic lambda$addFixers$8(D)D
private static synthetic lambda$addFixers$7(Ljava/lang/String;)Ljava/lang/String;
private static synthetic lambda$addFixers$6(Ljava/util/Map;Ljava/lang/String;)Ljava/lang/String;
private static synthetic lambda$addFixers$5(Lit/unimi/dsi/fastutil/ints/Int2ObjectOpenHashMap;)V
private static synthetic lambda$addFixers$4(Lcom/google/common/collect/ImmutableMap;Ljava/lang/String;)Ljava/lang/String;
private static synthetic lambda$addFixers$3(Lit/unimi/dsi/fastutil/ints/Int2ObjectOpenHashMap;)V
private static synthetic lambda$addFixers$2(Ljava/lang/String;)Ljava/lang/String;
private static synthetic lambda$addFixers$1(Ljava/lang/String;)Ljava/lang/String;
private static synthetic lambda$addFixers$0(Ljava/lang/String;)Ljava/lang/String;
static <clinit>()V
```
