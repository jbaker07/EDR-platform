---
type: "interface"
fqcn: "net.minecraft.server.Bootstrap"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.Bootstrap

System: [[20-Systems/net.minecraft.server|net.minecraft.server]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| injects_into | `bootStrap` | `@Inject at INVOKE Lnet/minecraft/server/Bootstrap;wrapStreams()V` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| wraps | `bootStrap` | `@Redirect at INVOKE Lnet/minecraft/core/registries/BuiltInRegistries;bootStrap()` | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (20, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.server.Bootstrap {
    public static final java.io.PrintStream STDOUT;
    private static volatile boolean isBootstrapped;
    private static final org.slf4j.Logger LOGGER;
    public static final java.util.concurrent.atomic.AtomicLong bootstrapDuration;
    public net.minecraft.server.Bootstrap();
    public static void bootStrap();
    private static <T> void checkTranslations(net.minecraft.locale.Language, java.lang.Iterable<T>, java.util.function.Function<T, java.lang.String>, java.util.Set<java.lang.String>);
    private static void checkGameruleTranslations(net.minecraft.locale.Language, java.util.Set<java.lang.String>);
    public static java.util.Set<java.lang.String> getMissingTranslations(net.minecraft.locale.Language);
    public static void checkBootstrapCalled(java.util.function.Supplier<java.lang.String>);
    private static java.lang.RuntimeException createBootstrapException(java.util.function.Supplier<java.lang.String>);
    public static void validate();
    private static void wrapStreams();
    public static void realStdoutPrintln(java.lang.String);
    public static void shutdownStdout();
    private static void lambda$validate$1(java.lang.String);
    private static java.lang.String lambda$validate$0();
    private static java.lang.String lambda$getMissingTranslations$0(net.minecraft.resources.Identifier);
    private static void lambda$checkTranslations$0(java.util.function.Function, net.minecraft.locale.Language, java.util.Set, java.lang.Object);
    static {};
}
```
