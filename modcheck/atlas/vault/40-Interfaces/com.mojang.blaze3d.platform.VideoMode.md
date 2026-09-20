---
type: "interface"
fqcn: "com.mojang.blaze3d.platform.VideoMode"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# com.mojang.blaze3d.platform.VideoMode

System: [[20-Systems/com.mojang.blaze3d.platform|com.mojang.blaze3d.platform]]

`class` public final; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getHeight` | `()I` | exact | invokevirtual@175 in `WindowMixin.fabric_resize` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getHeight` | `()I` | exact | invokevirtual@189 in `WindowMixin.fabric_resize` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getWidth` | `()I` | exact | invokevirtual@121 in `WindowMixin.fabric_resize` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getWidth` | `()I` | exact | invokevirtual@135 in `WindowMixin.fabric_resize` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (9 fields, 18 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final LOGGER : Lorg/slf4j/Logger;
public static final CODEC : Lcom/mojang/serialization/Codec;
private static final GSON : Lcom/google/gson/Gson;
private final width : I
private final height : I
private final redBits : I
private final greenBits : I
private final blueBits : I
private final refreshRate : F
public <init>(IIIIII)V
private <init>(IIIIIF)V
public <init>(Lorg/lwjgl/sdl/SDL_DisplayMode;)V
public static read(Ljava/lang/String;)Ljava/util/Optional;
public getWidth()I
public getHeight()I
public getRedBits()I
public getGreenBits()I
public getBlueBits()I
public getRefreshRate()F
public refreshRateLabel()Ljava/lang/String;
public equals(Ljava/lang/Object;)Z
public hashCode()I
public toString()Ljava/lang/String;
public write()Ljava/lang/String;
private static synthetic lambda$read$0(Ljava/lang/String;Ljava/lang/String;)V
private static synthetic lambda$static$0(Lcom/mojang/serialization/codecs/RecordCodecBuilder$Instance;)Lcom/mojang/datafixers/kinds/App;
static <clinit>()V
```
