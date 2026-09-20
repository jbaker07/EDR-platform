---
type: "interface"
fqcn: "net.minecraft.util.Util$OS"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.Util$OS

System: [[20-Systems/net.minecraft.util|net.minecraft.util]]

`enum` public final; extends `java/lang/Enum`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `OSX` | `Lnet/minecraft/util/Util$OS;` | exact | getstatic@13 in `TestInputImpl.<clinit>` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `WINDOWS` | `Lnet/minecraft/util/Util$OS;` | exact | getstatic@15 in `DedicatedServerImplUtil.lambda$static$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (7 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final LINUX : Lnet/minecraft/util/Util$OS;
public static final SOLARIS : Lnet/minecraft/util/Util$OS;
public static final WINDOWS : Lnet/minecraft/util/Util$OS;
public static final OSX : Lnet/minecraft/util/Util$OS;
public static final UNKNOWN : Lnet/minecraft/util/Util$OS;
private final telemetryName : Ljava/lang/String;
private static final synthetic $VALUES : [Lnet/minecraft/util/Util$OS;
public static values()[Lnet/minecraft/util/Util$OS;
public static valueOf(Ljava/lang/String;)Lnet/minecraft/util/Util$OS;
private <init>(Ljava/lang/String;ILjava/lang/String;)V
public telemetryName()Ljava/lang/String;
private static synthetic $values()[Lnet/minecraft/util/Util$OS;
static <clinit>()V
```
