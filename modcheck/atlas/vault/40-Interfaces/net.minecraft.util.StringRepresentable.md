---
type: "interface"
fqcn: "net.minecraft.util.StringRepresentable"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.StringRepresentable

System: [[20-Systems/net.minecraft.util|net.minecraft.util]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `fromEnum(Ljava/util/function/Supplier;)Lnet/minecraft/util/StringRep` | `` | both | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |

## Declared members (12, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.util.StringRepresentable {
    public static final int PRE_BUILT_MAP_THRESHOLD;
    public abstract java.lang.String getSerializedName();
    public static <E extends java.lang.Enum<E> & net.minecraft.util.StringRepresentable> net.minecraft.util.StringRepresentable$EnumCodec<E> fromEnum(java.util.function.Supplier<E[]>);
    public static <E extends java.lang.Enum<E> & net.minecraft.util.StringRepresentable> net.minecraft.util.StringRepresentable$EnumCodec<E> fromEnumWithMapping(java.util.function.Supplier<E[]>, java.util.function.Function<java.lang.String, java.lang.String>);
    public static <T extends net.minecraft.util.StringRepresentable> com.mojang.serialization.Codec<T> fromValues(java.util.function.Supplier<T[]>);
    public static <T extends net.minecraft.util.StringRepresentable> java.util.function.Function<java.lang.String, T> createNameLookup(T[]);
    public static <T> java.util.function.Function<java.lang.String, T> createNameLookup(T[], java.util.function.Function<T, java.lang.String>);
    public static com.mojang.serialization.Keyable keys(net.minecraft.util.StringRepresentable[]);
    private static java.lang.Object lambda$createNameLookup$1(java.lang.Object[], java.util.function.Function, java.lang.String);
    private static java.lang.Object lambda$createNameLookup$0(java.lang.Object);
    private static java.lang.String lambda$fromEnumWithMapping$0(java.util.function.Function, java.lang.Enum);
    private static java.lang.String lambda$fromEnum$0(java.lang.String);
}
```
