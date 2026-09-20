---
type: "interface"
fqcn: "net.minecraft.client.input.KeyEvent"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.input.KeyEvent

System: [[20-Systems/net.minecraft.client.input|net.minecraft.client.input]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(III)V` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `key()I` | `` | client | [[30-Mechanisms/fabric-creative-tab-api-v1|fabric-creative-tab-api-v1]] | direct_reference |

## Declared members (12, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.client.input.KeyEvent extends java.lang.Record implements net.minecraft.client.input.InputWithModifiers {
    private final int key;
    private final int keycode;
    private final int modifiers;
    public net.minecraft.client.input.KeyEvent(int, int, int);
    public int input();
    public int shortcutKey();
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public int key();
    public int keycode();
    public int modifiers();
}
```
