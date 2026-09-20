---
type: "interface"
fqcn: "net.minecraft.client.input.CharacterEvent"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.input.CharacterEvent

System: [[20-Systems/net.minecraft.client.input|net.minecraft.client.input]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(I)V` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (8, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.client.input.CharacterEvent extends java.lang.Record {
    private final int codepoint;
    public net.minecraft.client.input.CharacterEvent(int);
    public java.lang.String codepointAsString();
    public boolean isAllowedChatCharacter();
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public int codepoint();
}
```
