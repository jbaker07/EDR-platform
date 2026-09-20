---
type: "interface"
fqcn: "net.minecraft.util.StringRepresentable$EnumCodec"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.StringRepresentable$EnumCodec

System: [[20-Systems/net.minecraft.util|net.minecraft.util]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `fieldOf(Ljava/lang/String;)Lcom/mojang/serialization/MapCodec;` | `` | both | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |

## Declared members (6, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.util.StringRepresentable$EnumCodec<E extends java.lang.Enum<E> & net.minecraft.util.StringRepresentable> extends net.minecraft.util.StringRepresentable$StringRepresentableCodec<E> {
    private final java.util.function.Function<java.lang.String, E> resolver;
    public net.minecraft.util.StringRepresentable$EnumCodec(E[], java.util.function.Function<java.lang.String, E>);
    public E byName(java.lang.String);
    public E byName(java.lang.String, E);
    public E byName(java.lang.String, java.util.function.Supplier<? extends E>);
    private static int lambda$new$0(java.lang.Enum);
}
```
