---
type: "interface"
fqcn: "net.minecraft.client.renderer.Rect2i"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.Rect2i

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| calls | `"<init>"(IIII)V` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `"<init>"(IIII)V` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getHeight()I` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getWidth()I` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getX()I` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getY()I` | `` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (16, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.client.renderer.Rect2i {
    private int xPos;
    private int yPos;
    private int width;
    private int height;
    public net.minecraft.client.renderer.Rect2i(int, int, int, int);
    public net.minecraft.client.renderer.Rect2i intersect(net.minecraft.client.renderer.Rect2i);
    public int getX();
    public int getY();
    public void setX(int);
    public void setY(int);
    public int getWidth();
    public int getHeight();
    public void setWidth(int);
    public void setHeight(int);
    public void setPosition(int, int);
    public boolean contains(int, int);
}
```
