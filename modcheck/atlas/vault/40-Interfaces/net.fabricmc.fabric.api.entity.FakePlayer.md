---
type: "interface"
fqcn: "net.fabricmc.fabric.api.entity.FakePlayer"
module: "fabric-events-interaction-v0"
sha256: "f57dd8df1d78cbcaf6ee1073aebd64e5c959cc7224002098e653832f0ecd7de8"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.fabric.api.entity.FakePlayer

Module: [[30-Mechanisms/fabric-events-interaction-v0|fabric-events-interaction-v0]] -- kind: class

```java
public static final java.util.UUID DEFAULT_UUID
public static net.fabricmc.fabric.api.entity.FakePlayer get(net.minecraft.server.level.ServerLevel)
public static net.fabricmc.fabric.api.entity.FakePlayer get(net.minecraft.server.level.ServerLevel, com.mojang.authlib.GameProfile)
protected net.fabricmc.fabric.api.entity.FakePlayer(net.minecraft.server.level.ServerLevel, com.mojang.authlib.GameProfile)
public void tick()
public void updateOptions(net.minecraft.server.level.ClientInformation)
public void awardStat(net.minecraft.stats.Stat<?>, int)
public void resetStat(net.minecraft.stats.Stat<?>)
public boolean isInvulnerableTo(net.minecraft.server.level.ServerLevel, net.minecraft.world.damagesource.DamageSource)
public net.minecraft.world.scores.PlayerTeam getTeam()
public boolean startSleeping(net.minecraft.core.BlockPos)
public boolean startRiding(net.minecraft.world.entity.Entity, boolean, boolean)
public void openTextEdit(net.minecraft.world.level.block.entity.SignBlockEntity, net.minecraft.world.level.block.entity.SignTextSlot)
public java.util.OptionalInt openMenu(net.minecraft.world.MenuProvider)
public void openHorseInventory(net.minecraft.world.entity.animal.equine.AbstractHorse, net.minecraft.world.Container)
static {}
```
