---
type: "interface"
fqcn: "net.minecraft.advancements.AdvancementNode"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.advancements.AdvancementNode

System: [[20-Systems/net.minecraft.advancements|net.minecraft.advancements]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `holder()Lnet/minecraft/advancements/AdvancementHolder;` | `` | client | [[30-Mechanisms/fabric-rendering-v1|fabric-rendering-v1]] | direct_reference |

## Declared members (21, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.advancements.AdvancementNode {
    private final net.minecraft.advancements.AdvancementHolder holder;
    private final net.minecraft.advancements.AdvancementNode parent;
    private final java.util.Set<net.minecraft.advancements.AdvancementNode> children;
    private float x;
    private float y;
    public net.minecraft.advancements.AdvancementNode(net.minecraft.advancements.AdvancementHolder, net.minecraft.advancements.AdvancementNode);
    public net.minecraft.advancements.Advancement advancement();
    public net.minecraft.advancements.AdvancementHolder holder();
    public boolean isTask();
    public boolean isRoot();
    public net.minecraft.advancements.AdvancementNode parent();
    public net.minecraft.advancements.AdvancementNode root();
    public static net.minecraft.advancements.AdvancementNode getRoot(net.minecraft.advancements.AdvancementNode);
    public java.lang.Iterable<net.minecraft.advancements.AdvancementNode> children();
    public void addChild(net.minecraft.advancements.AdvancementNode);
    public void setLocation(float, float);
    public float x();
    public float y();
    public boolean equals(java.lang.Object);
    public int hashCode();
    public java.lang.String toString();
}
```
