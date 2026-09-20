---
type: "interface"
fqcn: "net.minecraft.client.resources.model.geometry.ItemQuads"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.resources.model.geometry.ItemQuads

System: [[20-Systems/net.minecraft.client.resources|net.minecraft.client.resources]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `all()Ljava/util/List;` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `solid()Ljava/util/List;` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |
| calls | `translucent()Ljava/util/List;` | `` | client | [[30-Mechanisms/fabric-renderer-api-v1|fabric-renderer-api-v1]] | direct_reference |

## Declared members (14, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public final class net.minecraft.client.resources.model.geometry.ItemQuads extends java.lang.Record {
    private final java.util.List<net.minecraft.client.resources.model.geometry.BakedQuad> all;
    private final java.util.List<net.minecraft.client.resources.model.geometry.BakedQuad> solid;
    private final java.util.List<net.minecraft.client.resources.model.geometry.BakedQuad> translucent;
    public static final net.minecraft.client.resources.model.geometry.ItemQuads EMPTY;
    public net.minecraft.client.resources.model.geometry.ItemQuads(java.util.List<net.minecraft.client.resources.model.geometry.BakedQuad>, java.util.List<net.minecraft.client.resources.model.geometry.BakedQuad>, java.util.List<net.minecraft.client.resources.model.geometry.BakedQuad>);
    public static net.minecraft.client.resources.model.geometry.ItemQuads split(java.util.List<net.minecraft.client.resources.model.geometry.BakedQuad>);
    public boolean isEmpty();
    public final java.lang.String toString();
    public final int hashCode();
    public final boolean equals(java.lang.Object);
    public java.util.List<net.minecraft.client.resources.model.geometry.BakedQuad> all();
    public java.util.List<net.minecraft.client.resources.model.geometry.BakedQuad> solid();
    public java.util.List<net.minecraft.client.resources.model.geometry.BakedQuad> translucent();
    static {};
}
```
