---
type: "interface"
fqcn: "net.minecraft.server.level.ChunkResult"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.level.ChunkResult

System: [[20-Systems/net.minecraft.server.level|net.minecraft.server.level]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `ifSuccess(Ljava/util/function/Consumer;)Lnet/minecraft/server/level/C` | `` | both | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |

## Declared members (11, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public interface net.minecraft.server.level.ChunkResult<T> {
    public static <T> net.minecraft.server.level.ChunkResult<T> of(T);
    public static <T> net.minecraft.server.level.ChunkResult<T> error(java.lang.String);
    public static <T> net.minecraft.server.level.ChunkResult<T> error(java.util.function.Supplier<java.lang.String>);
    public abstract boolean isSuccess();
    public abstract T orElse(T);
    public static <R> R orElse(net.minecraft.server.level.ChunkResult<? extends R>, R);
    public abstract java.lang.String getError();
    public abstract net.minecraft.server.level.ChunkResult<T> ifSuccess(java.util.function.Consumer<T>);
    public abstract <R> net.minecraft.server.level.ChunkResult<R> map(java.util.function.Function<T, R>);
    public abstract <E extends java.lang.Throwable> T orElseThrow(java.util.function.Supplier<E>) throws E;
    private static java.lang.String lambda$error$0(java.lang.String);
}
```
