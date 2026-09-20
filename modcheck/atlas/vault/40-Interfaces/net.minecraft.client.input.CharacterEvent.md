---
type: "interface"
fqcn: "net.minecraft.client.input.CharacterEvent"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.input.CharacterEvent

System: [[20-Systems/net.minecraft.client.input|net.minecraft.client.input]]

`record` public final; extends `java/lang/Record`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(I)V` | exact | invokespecial@19 in `TestInputImpl.lambda$typeChars$1` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `<init>` | `(I)V` | exact | invokespecial@19 in `TestInputImpl.lambda$typeChar$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (1 fields, 7 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final codepoint : I
public <init>(I)V
public codepointAsString()Ljava/lang/String;
public isAllowedChatCharacter()Z
public final toString()Ljava/lang/String;
public final hashCode()I
public final equals(Ljava/lang/Object;)Z
public codepoint()I
```
