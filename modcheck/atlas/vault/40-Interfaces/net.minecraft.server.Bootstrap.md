---
type: "interface"
fqcn: "net.minecraft.server.Bootstrap"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.Bootstrap

System: [[20-Systems/net.minecraft.server|net.minecraft.server]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `bootStrap` | `()V` | name_only | @Inject at ['INVOKE'] | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| wraps | `bootStrap` | `()V` | name_only | @Redirect at ['INVOKE'] | both | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (4 fields, 16 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final STDOUT : Ljava/io/PrintStream;
private static isBootstrapped : Z
private static final LOGGER : Lorg/slf4j/Logger;
public static final bootstrapDuration : Ljava/util/concurrent/atomic/AtomicLong;
public <init>()V
public static bootStrap()V
private static checkTranslations(Lnet/minecraft/locale/Language;Ljava/lang/Iterable;Ljava/util/function/Function;Ljava/util/Set;)V
private static checkGameruleTranslations(Lnet/minecraft/locale/Language;Ljava/util/Set;)V
public static getMissingTranslations(Lnet/minecraft/locale/Language;)Ljava/util/Set;
public static checkBootstrapCalled(Ljava/util/function/Supplier;)V
private static createBootstrapException(Ljava/util/function/Supplier;)Ljava/lang/RuntimeException;
public static validate()V
private static wrapStreams()V
public static realStdoutPrintln(Ljava/lang/String;)V
public static shutdownStdout()V
private static synthetic lambda$validate$1(Ljava/lang/String;)V
private static synthetic lambda$validate$0()Ljava/lang/String;
private static synthetic lambda$getMissingTranslations$0(Lnet/minecraft/resources/Identifier;)Ljava/lang/String;
private static synthetic lambda$checkTranslations$0(Ljava/util/function/Function;Lnet/minecraft/locale/Language;Ljava/util/Set;Ljava/lang/Object;)V
static <clinit>()V
```
