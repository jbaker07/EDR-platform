---
type: "interface"
fqcn: "net.minecraft.Optionull"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.Optionull

System: [[20-Systems/net.minecraft|net.minecraft]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `map(Ljava/lang/Object;Ljava/util/function/Function;)Ljava/lang/` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (17, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.Optionull {
    public net.minecraft.Optionull();
    public static <T> T orElse(T, T);
    public static <T, R> R map(T, java.util.function.Function<T, R>);
    public static <T, R> R mapOrDefault(T, java.util.function.Function<T, R>, R);
    public static <T, R> R mapOrElse(T, java.util.function.Function<T, R>, java.util.function.Supplier<R>);
    public static <T> T first(java.util.Collection<T>);
    public static <T> T firstOrDefault(java.util.Collection<T>, T);
    public static <T> T firstOrElse(java.util.Collection<T>, java.util.function.Supplier<T>);
    public static <T> boolean isNullOrEmpty(T[]);
    public static boolean isNullOrEmpty(boolean[]);
    public static boolean isNullOrEmpty(byte[]);
    public static boolean isNullOrEmpty(char[]);
    public static boolean isNullOrEmpty(short[]);
    public static boolean isNullOrEmpty(int[]);
    public static boolean isNullOrEmpty(long[]);
    public static boolean isNullOrEmpty(float[]);
    public static boolean isNullOrEmpty(double[]);
}
```
