---
type: "interface"
fqcn: "net.minecraft.world.entity.player.Abilities"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.world.entity.player.Abilities

System: [[20-Systems/net.minecraft.world.entity|net.minecraft.world.entity]]

## How Fabric API modules touch this type

| relation | member | operation | environment | by | evidence |
|---|---|---|---|---|---|
| reads | `instabuildZ` | `` | client | [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] | direct_reference |

## Declared members (21, all visibilities)

From `minecraft-merged` `5918174887871ab0` via `javap -p`. Inherited members are not listed here.

```java
public class net.minecraft.world.entity.player.Abilities {
    private static final boolean DEFAULT_INVULNERABLE;
    private static final boolean DEFAULY_FLYING;
    private static final boolean DEFAULT_MAY_FLY;
    private static final boolean DEFAULT_INSTABUILD;
    private static final boolean DEFAULT_MAY_BUILD;
    private static final float DEFAULT_FLYING_SPEED;
    private static final float DEFAULT_WALKING_SPEED;
    public boolean invulnerable;
    public boolean flying;
    public boolean mayfly;
    public boolean instabuild;
    public boolean mayBuild;
    private float flyingSpeed;
    private float walkingSpeed;
    public net.minecraft.world.entity.player.Abilities();
    public float getFlyingSpeed();
    public void setFlyingSpeed(float);
    public float getWalkingSpeed();
    public void setWalkingSpeed(float);
    public net.minecraft.world.entity.player.Abilities$Packed pack();
    public void apply(net.minecraft.world.entity.player.Abilities$Packed);
}
```
