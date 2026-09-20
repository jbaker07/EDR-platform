---
type: "interface"
fqcn: "net.minecraft.client.resources.language.I18n"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.resources.language.I18n

System: [[20-Systems/net.minecraft.client.resources|net.minecraft.client.resources]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `get(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;` | `` | client | [[30-Mechanisms/fabric-game-rule-api-v1|fabric-game-rule-api-v1]] | direct_reference |

## Declared members (2, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.resources.language.I18n {
    private net.minecraft.client.resources.language.I18n();
    public static java.lang.String get(java.lang.String, java.lang.Object...);
}
```
