---
type: "interface"
fqcn: "net.minecraft.client.input.MouseButtonInfo"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.input.MouseButtonInfo

System: [[20-Systems/net.minecraft.client.input|net.minecraft.client.input]]

`record` public final; extends `java/lang/Record`; implements `net/minecraft/client/input/InputWithModifiers`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(II)V` | exact | invokespecial@103 in `TestInputImpl.pressOrReleaseKey` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `<init>` | `(II)V` | exact | invokespecial@6 in `ClientGameTestContextImpl.pressMatchingButton` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (2 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final button : I
private final modifiers : I
public <init>(II)V
public input()I
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public button()I
public modifiers()I
```
