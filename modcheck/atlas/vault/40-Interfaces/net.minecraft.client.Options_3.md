---
type: "interface"
fqcn: "net.minecraft.client.Options$3"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.Options$3

System: [[20-Systems/net.minecraft.client|net.minecraft.client]]

`class` ; extends `java/lang/Object`; implements `net/minecraft/client/Options$FieldAccess`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| injects_into | `process` | `(Ljava/lang/String;Ljava/lang/Object;Ljava/util/function/Function;Ljav` | exact | @ModifyArg at ['INVOKE'] | client | [[30-Mechanisms/fabric-resource-loader-v1|fabric-resource-loader-v1]] | direct_reference |

## Declared members (1 fields, 10 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
final synthetic val$writer : Ljava/io/PrintWriter;
 <init>(Lnet/minecraft/client/Options;Ljava/io/PrintWriter;)V
public writePrefix(Ljava/lang/String;)V
public process(Ljava/lang/String;Lnet/minecraft/client/OptionInstance;)V
public process(Ljava/lang/String;I)I
public process(Ljava/lang/String;Z)Z
public process(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/String;
public process(Ljava/lang/String;F)F
public process(Ljava/lang/String;Ljava/lang/Object;Ljava/util/function/Function;Ljava/util/function/Function;)Ljava/lang/Object;
private synthetic lambda$process$1(Ljava/lang/String;Ljava/io/PrintWriter;Lcom/google/gson/JsonElement;)V
private static synthetic lambda$process$0(Lnet/minecraft/client/OptionInstance;Lcom/mojang/serialization/DataResult$Error;)V
```
