---
type: "interface"
fqcn: "net.minecraft.util.datafix.DataFixers"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.datafix.DataFixers

System: [[20-Systems/net.minecraft.util.datafix|net.minecraft.util.datafix]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `getDataFixer()Lcom/mojang/datafixers/DataFixer;` | `` | both | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (35, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.util.datafix.DataFixers {
    private static final java.util.function.BiFunction<java.lang.Integer, com.mojang.datafixers.schemas.Schema, com.mojang.datafixers.schemas.Schema> SAME;
    private static final java.util.function.BiFunction<java.lang.Integer, com.mojang.datafixers.schemas.Schema, com.mojang.datafixers.schemas.Schema> SAME_NAMESPACED;
    private static final com.mojang.datafixers.DataFixerBuilder$Result DATA_FIXER;
    private static final net.minecraft.util.filefix.FileFixerUpper FILE_FIXER;
    public static final int BLENDING_VERSION;
    private net.minecraft.util.datafix.DataFixers();
    public static com.mojang.datafixers.DataFixer getDataFixer();
    public static net.minecraft.util.filefix.FileFixerUpper getFileFixer();
    public static java.util.concurrent.CompletableFuture<?> optimize(java.util.Set<com.mojang.datafixers.DSL$TypeReference>);
    private static void addFixers(com.mojang.datafixers.DataFixerBuilder, net.minecraft.util.filefix.FileFixerUpper$Builder);
    private static java.util.function.UnaryOperator<java.lang.String> createRenamerNoNamespace(java.util.Map<java.lang.String, java.lang.String>);
    private static java.util.function.UnaryOperator<java.lang.String> createRenamer(java.util.Map<java.lang.String, java.lang.String>);
    private static java.util.function.UnaryOperator<java.lang.String> createRenamer(java.lang.String, java.lang.String);
    private static java.lang.String lambda$createRenamer$1(java.lang.String, java.lang.String, java.lang.String);
    private static java.lang.String lambda$createRenamer$0(java.util.Map, java.lang.String);
    private static java.lang.String lambda$createRenamerNoNamespace$0(java.util.Map, java.lang.String);
    private static com.mojang.serialization.Dynamic lambda$addFixers$17(com.mojang.serialization.Dynamic);
    private static com.mojang.serialization.Dynamic lambda$addFixers$16(com.mojang.serialization.Dynamic);
    private static java.lang.String lambda$addFixers$15(java.lang.String);
    private static java.lang.String lambda$addFixers$14(java.lang.String);
    private static double lambda$addFixers$13(double);
    private static double lambda$addFixers$12(double);
    private static double lambda$addFixers$11(double);
    private static double lambda$addFixers$10(double);
    private static double lambda$addFixers$9(double);
    private static double lambda$addFixers$8(double);
    private static java.lang.String lambda$addFixers$7(java.lang.String);
    private static java.lang.String lambda$addFixers$6(java.util.Map, java.lang.String);
    private static void lambda$addFixers$5(it.unimi.dsi.fastutil.ints.Int2ObjectOpenHashMap);
    private static java.lang.String lambda$addFixers$4(com.google.common.collect.ImmutableMap, java.lang.String);
    private static void lambda$addFixers$3(it.unimi.dsi.fastutil.ints.Int2ObjectOpenHashMap);
    private static java.lang.String lambda$addFixers$2(java.lang.String);
    private static java.lang.String lambda$addFixers$1(java.lang.String);
    private static java.lang.String lambda$addFixers$0(java.lang.String);
    static {};
}
```
