---
type: "interface"
fqcn: "net.minecraft.client.CloudStatus"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.CloudStatus

System: [[20-Systems/net.minecraft.client|net.minecraft.client]]

`enum` public final; extends `java/lang/Enum`; implements `net/minecraft/util/StringRepresentable`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `OFF` | `Lnet/minecraft/client/CloudStatus;` | exact | getstatic@11 in `ClientGameTestContextImpl.initGameOptions` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (7 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final OFF : Lnet/minecraft/client/CloudStatus;
public static final FAST : Lnet/minecraft/client/CloudStatus;
public static final FANCY : Lnet/minecraft/client/CloudStatus;
public static final CODEC : Lcom/mojang/serialization/Codec;
private final legacyName : Ljava/lang/String;
private final caption : Lnet/minecraft/network/chat/Component;
private static final synthetic $VALUES : [Lnet/minecraft/client/CloudStatus;
public static values()[Lnet/minecraft/client/CloudStatus;
public static valueOf(Ljava/lang/String;)Lnet/minecraft/client/CloudStatus;
private <init>(Ljava/lang/String;ILjava/lang/String;Ljava/lang/String;)V
public caption()Lnet/minecraft/network/chat/Component;
public getSerializedName()Ljava/lang/String;
private static synthetic $values()[Lnet/minecraft/client/CloudStatus;
static <clinit>()V
```
