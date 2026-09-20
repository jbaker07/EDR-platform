---
type: "interface"
fqcn: "net.minecraft.util.datafix.schemas.V2832"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.datafix.schemas.V2832

System: [[20-Systems/net.minecraft.util.datafix|net.minecraft.util.datafix]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| wraps | `lambda$registerTypes$2` | `@Redirect at INVOKE Lcom/mojang/datafixers/DSL;taggedChoiceLazy(Ljava/lang/Strin` | both | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |
| wraps | `lambda$registerTypes$4` | `@Redirect at INVOKE Lcom/mojang/datafixers/DSL;taggedChoiceLazy(Ljava/lang/Strin` | both | [[30-Mechanisms/fabric-dimensions-v1|fabric-dimensions-v1]] | direct_reference |

## Declared members (10, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.util.datafix.schemas.V2832 extends net.minecraft.util.datafix.schemas.NamespacedSchema {
    public net.minecraft.util.datafix.schemas.V2832(int, com.mojang.datafixers.schemas.Schema);
    public void registerTypes(com.mojang.datafixers.schemas.Schema, java.util.Map<java.lang.String, java.util.function.Supplier<com.mojang.datafixers.types.templates.TypeTemplate>>, java.util.Map<java.lang.String, java.util.function.Supplier<com.mojang.datafixers.types.templates.TypeTemplate>>);
    private static com.mojang.datafixers.types.templates.TypeTemplate lambda$registerTypes$2(com.mojang.datafixers.schemas.Schema);
    private static com.mojang.datafixers.types.templates.TypeTemplate lambda$registerTypes$4(com.mojang.datafixers.schemas.Schema);
    private static com.mojang.datafixers.types.templates.TypeTemplate lambda$registerTypes$7(com.mojang.datafixers.schemas.Schema);
    private static com.mojang.datafixers.types.templates.TypeTemplate lambda$registerTypes$6(com.mojang.datafixers.schemas.Schema);
    private static com.mojang.datafixers.types.templates.TypeTemplate lambda$registerTypes$5(com.mojang.datafixers.schemas.Schema);
    private static com.mojang.datafixers.types.templates.TypeTemplate lambda$registerTypes$3(com.mojang.datafixers.schemas.Schema);
    private static com.mojang.datafixers.types.templates.TypeTemplate lambda$registerTypes$1();
    private static com.mojang.datafixers.types.templates.TypeTemplate lambda$registerTypes$0(com.mojang.datafixers.schemas.Schema);
}
```
