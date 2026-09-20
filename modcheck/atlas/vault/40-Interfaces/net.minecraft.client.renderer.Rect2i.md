---
type: "interface"
fqcn: "net.minecraft.client.renderer.Rect2i"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.renderer.Rect2i

System: [[20-Systems/net.minecraft.client.renderer|net.minecraft.client.renderer]]

`class` public; extends `java/lang/Object`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(IIII)V` | exact | invokespecial@21 in `ClientGameTestContextImpl.lambda$doAssertScreenshotContains$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `<init>` | `(IIII)V` | exact | invokespecial@67 in `TestScreenshotComparisonOptionsImpl.withRegion` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getHeight` | `()I` | exact | invokevirtual@53 in `ClientGameTestContextImpl.lambda$doAssertScreenshotContains$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getHeight` | `()I` | exact | invokevirtual@84 in `ClientGameTestContextImpl.lambda$doAssertScreenshotContains$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getHeight` | `()I` | exact | invokevirtual@107 in `ClientGameTestContextImpl.lambda$doAssertScreenshotContains$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getWidth` | `()I` | exact | invokevirtual@37 in `ClientGameTestContextImpl.lambda$doAssertScreenshotContains$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getWidth` | `()I` | exact | invokevirtual@80 in `ClientGameTestContextImpl.lambda$doAssertScreenshotContains$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getWidth` | `()I` | exact | invokevirtual@103 in `ClientGameTestContextImpl.lambda$doAssertScreenshotContains$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getX` | `()I` | exact | invokevirtual@33 in `ClientGameTestContextImpl.lambda$doAssertScreenshotContains$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getX` | `()I` | exact | invokevirtual@95 in `ClientGameTestContextImpl.lambda$doAssertScreenshotContains$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getX` | `()I` | exact | invokevirtual@162 in `ClientGameTestContextImpl.lambda$doAssertScreenshotContains$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getX` | `()I` | exact | invokevirtual@248 in `ClientGameTestContextImpl.lambda$doAssertScreenshotContains$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getX` | `()I` | exact | invokevirtual@359 in `ClientGameTestContextImpl.lambda$doAssertScreenshotContains$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getY` | `()I` | exact | invokevirtual@49 in `ClientGameTestContextImpl.lambda$doAssertScreenshotContains$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getY` | `()I` | exact | invokevirtual@99 in `ClientGameTestContextImpl.lambda$doAssertScreenshotContains$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getY` | `()I` | exact | invokevirtual@166 in `ClientGameTestContextImpl.lambda$doAssertScreenshotContains$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getY` | `()I` | exact | invokevirtual@252 in `ClientGameTestContextImpl.lambda$doAssertScreenshotContains$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |
| calls | `getY` | `()I` | exact | invokevirtual@363 in `ClientGameTestContextImpl.lambda$doAssertScreenshotContains$0` | unknown | [[30-Mechanisms/fabric-client-gametest-api-v1|fabric-client-gametest-api-v1]] | direct_reference |

## Declared members (4 fields, 12 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private xPos : I
private yPos : I
private width : I
private height : I
public <init>(IIII)V
public intersect(Lnet/minecraft/client/renderer/Rect2i;)Lnet/minecraft/client/renderer/Rect2i;
public getX()I
public getY()I
public setX(I)V
public setY(I)V
public getWidth()I
public getHeight()I
public setWidth(I)V
public setHeight(I)V
public setPosition(II)V
public contains(II)Z
```
