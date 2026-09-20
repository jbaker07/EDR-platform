---
type: "interface"
fqcn: "net.minecraft.sounds.SoundSource"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.sounds.SoundSource

System: [[20-Systems/net.minecraft.sounds|net.minecraft.sounds]]

`enum` public final; extends `java/lang/Enum`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| reads | `BLOCKS` | `Lnet/minecraft/sounds/SoundSource;` | exact | getstatic@81 in `ComposterWrapper.onFinalCommit` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |
| reads | `MUSIC` | `Lnet/minecraft/sounds/SoundSource;` | exact | getstatic@56 in `ClientGameTestContextImpl.initGameOptions` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| reads | `PLAYERS` | `Lnet/minecraft/sounds/SoundSource;` | exact | getstatic@264 in `FluidStorageUtil.moveWithSound` | unknown | [[30-Mechanisms/fabric-transfer-api-v1|fabric-transfer-api-v1]] | direct_reference |

## Declared members (13 fields, 6 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final MASTER : Lnet/minecraft/sounds/SoundSource;
public static final MUSIC : Lnet/minecraft/sounds/SoundSource;
public static final RECORDS : Lnet/minecraft/sounds/SoundSource;
public static final WEATHER : Lnet/minecraft/sounds/SoundSource;
public static final BLOCKS : Lnet/minecraft/sounds/SoundSource;
public static final HOSTILE : Lnet/minecraft/sounds/SoundSource;
public static final NEUTRAL : Lnet/minecraft/sounds/SoundSource;
public static final PLAYERS : Lnet/minecraft/sounds/SoundSource;
public static final AMBIENT : Lnet/minecraft/sounds/SoundSource;
public static final VOICE : Lnet/minecraft/sounds/SoundSource;
public static final UI : Lnet/minecraft/sounds/SoundSource;
private final name : Ljava/lang/String;
private static final synthetic $VALUES : [Lnet/minecraft/sounds/SoundSource;
public static values()[Lnet/minecraft/sounds/SoundSource;
public static valueOf(Ljava/lang/String;)Lnet/minecraft/sounds/SoundSource;
private <init>(Ljava/lang/String;ILjava/lang/String;)V
public getName()Ljava/lang/String;
private static synthetic $values()[Lnet/minecraft/sounds/SoundSource;
static <clinit>()V
```
