---
type: "interface"
fqcn: "net.minecraft.util.ProblemReporter$ScopedCollector"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.ProblemReporter$ScopedCollector

System: [[20-Systems/net.minecraft.util|net.minecraft.util]]

`class` public; extends `net/minecraft/util/ProblemReporter$Collector`; implements `java/lang/AutoCloseable`; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Lnet/minecraft/util/ProblemReporter$PathElement;Lorg/slf4j/Logger;)V` | exact | invokespecial@11 in `AttachmentSavedData$1.encode` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `<init>` | `(Lnet/minecraft/util/ProblemReporter$PathElement;Lorg/slf4j/Logger;)V` | exact | invokespecial@11 in `AttachmentSavedData$2.decode` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `<init>` | `(Lorg/slf4j/Logger;)V` | exact | invokespecial@50 in `SerializableChunkDataMixin.setAttachmentDataInChunk` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `<init>` | `(Lorg/slf4j/Logger;)V` | exact | invokespecial@7 in `SerializableChunkDataMixin.storeAttachmentNbtData` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `close` | `()V` | exact | invokevirtual@53 in `AttachmentSavedData$1.encode` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `close` | `()V` | exact | invokevirtual@63 in `AttachmentSavedData$1.encode` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `close` | `()V` | exact | invokevirtual@80 in `AttachmentSavedData$2.decode` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `close` | `()V` | exact | invokevirtual@89 in `AttachmentSavedData$2.decode` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `close` | `()V` | exact | invokevirtual@82 in `SerializableChunkDataMixin.setAttachmentDataInChunk` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `close` | `()V` | exact | invokevirtual@92 in `SerializableChunkDataMixin.setAttachmentDataInChunk` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `close` | `()V` | exact | invokevirtual@69 in `SerializableChunkDataMixin.storeAttachmentNbtData` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |
| calls | `close` | `()V` | exact | invokevirtual@78 in `SerializableChunkDataMixin.storeAttachmentNbtData` | unknown | [[30-Mechanisms/fabric-data-attachment-api-v1|fabric-data-attachment-api-v1]] | direct_reference |

## Declared members (1 fields, 3 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private final logger : Lorg/slf4j/Logger;
public <init>(Lorg/slf4j/Logger;)V
public <init>(Lnet/minecraft/util/ProblemReporter$PathElement;Lorg/slf4j/Logger;)V
public close()V
```
