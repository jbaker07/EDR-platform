---
type: "interface"
fqcn: "net.minecraft.network.VarInt"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.network.VarInt

System: [[20-Systems/net.minecraft.network|net.minecraft.network]]

`class` public; extends `java/lang/Object`; implements nothing; **changed by Loom processing** (see [[00-Scope/Processed_Jar_Diff]]).

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `getByteSize` | `(I)I` | exact | invokestatic@26 in `AttachmentSync.<clinit>` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `getByteSize` | `(I)I` | exact | invokestatic@9 in `PayloadTypeRegistryImpl.padAndSetMaxPacketSize` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `read` | `(Lio/netty/buffer/ByteBuf;)I` | exact | invokestatic@192 in `FabricPacketMerger.decode` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `read` | `(Lio/netty/buffer/ByteBuf;)I` | exact | invokestatic@210 in `FabricPacketMerger.decode` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `read` | `(Lio/netty/buffer/ByteBuf;)I` | exact | invokestatic@41 in `IdDispatchCodecMixin.decode` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |
| calls | `write` | `(Lio/netty/buffer/ByteBuf;I)Lio/netty/buffer/ByteBuf;` | exact | invokestatic@88 in `FabricPacketSplitter.genericPacketSplitter` | unknown | [[30-Mechanisms/fabric-networking-api-v1|fabric-networking-api-v1]] | direct_reference |

## Declared members (4 fields, 5 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
public static final MAX_VARINT_SIZE : I
private static final DATA_BITS_MASK : I
private static final CONTINUATION_BIT_MASK : I
private static final DATA_BITS_PER_BYTE : I
public <init>()V
public static getByteSize(I)I
public static hasContinuationBit(B)Z
public static read(Lio/netty/buffer/ByteBuf;)I
public static write(Lio/netty/buffer/ByteBuf;I)Lio/netty/buffer/ByteBuf;
```
