---
type: "interface"
fqcn: "net.minecraft.util.StrictJsonParser"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.StrictJsonParser

System: [[20-Systems/net.minecraft.util|net.minecraft.util]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `parse(Ljava/io/Reader;)Lcom/google/gson/JsonElement;` | `` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |
| calls | `parse(Ljava/io/Reader;)Lcom/google/gson/JsonElement;` | `` | unknown | [[30-Mechanisms/fabric-tag-api-v1|fabric-tag-api-v1]] | direct_reference |

## Declared members (3, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.util.StrictJsonParser {
    public net.minecraft.util.StrictJsonParser();
    public static com.google.gson.JsonElement parse(java.io.Reader) throws com.google.gson.JsonIOException, com.google.gson.JsonSyntaxException;
    public static com.google.gson.JsonElement parse(java.lang.String) throws com.google.gson.JsonSyntaxException;
}
```
