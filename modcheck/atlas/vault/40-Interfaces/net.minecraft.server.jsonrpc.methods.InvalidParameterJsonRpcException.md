---
type: "interface"
fqcn: "net.minecraft.server.jsonrpc.methods.InvalidParameterJsonRpcException"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.jsonrpc.methods.InvalidParameterJsonRpcException

System: [[20-Systems/net.minecraft.server.jsonrpc|net.minecraft.server.jsonrpc]]

`class` public; extends `java/lang/RuntimeException`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Ljava/lang/String;)V` | exact | invokespecial@36 in `GameRulesServiceGameRuleUpdateMixin.fabric_checkType` | unknown | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |

## Declared members (0 fields, 1 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public <init>(Ljava/lang/String;)V
```
