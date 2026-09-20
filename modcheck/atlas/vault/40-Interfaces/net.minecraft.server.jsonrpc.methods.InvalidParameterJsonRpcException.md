---
type: "interface"
fqcn: "net.minecraft.server.jsonrpc.methods.InvalidParameterJsonRpcException"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.server.jsonrpc.methods.InvalidParameterJsonRpcException

System: [[20-Systems/net.minecraft.server.jsonrpc|net.minecraft.server.jsonrpc]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(Ljava/lang/String;)V` | `` | both | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |

## Declared members (1, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.server.jsonrpc.methods.InvalidParameterJsonRpcException extends java.lang.RuntimeException {
    public net.minecraft.server.jsonrpc.methods.InvalidParameterJsonRpcException(java.lang.String);
}
```
