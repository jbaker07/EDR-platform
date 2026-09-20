---
type: "system"
package: "net.minecraft.util.filefix"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.util.filefix

73 classes (57 top-level) across 6 packages in the processed jar; 0 changed by Loom processing; 2 hooked by Fabric API.

## Hooked types

- [[40-Interfaces/net.minecraft.util.filefix.fixes.DimensionStorageFileFix|DimensionStorageFileFix]] -- injects_into:1 -- by fabric-data-attachment-api-v1
- [[40-Interfaces/net.minecraft.util.filefix.operations.FileFixOperations|FileFixOperations]] -- calls:1 -- by fabric-data-attachment-api-v1

## Declared inventory

### `net.minecraft.util.filefix` (10 top-level)

`AbortedFileFixException`, `AtomicMoveNotSupportedFileFixException`, `CanceledFileFixException`, `FailedCleanupFileFixException`, `FileFix`, `FileFixException`, `FileFixUtil`, `FileFixerUpper`, `FileSystemCapabilities`, `package-info`

### `net.minecraft.util.filefix.access` (11 top-level)

`ChunkNbt`, `CompressedNbt`, `FileAccess`, `FileAccessProvider`, `FileRelation`, `FileResourceType`, `FileResourceTypes`, `LevelDat`, `PlayerData`, `SavedDataNbt`, `package-info`

### `net.minecraft.util.filefix.fixes` (9 top-level)

[[40-Interfaces/net.minecraft.util.filefix.fixes.DimensionStorageFileFix|DimensionStorageFileFix]], `GeneratedStructuresRenameFileFix`, `LegacyStructureFileFix`, `LevelDatToSavedDataFileFix`, `PlayerStorageFileFix`, `ReenableSpectatorsGenerateChunksInHardcoreWorldsFileFix`, `RemoveObsoleteFilesFileFix`, `ResourcePackLocationFileFix`, `package-info`

### `net.minecraft.util.filefix.operations` (9 top-level)

`ApplyInFolders`, `DeleteFileOrEmptyDirectory`, `FileFixOperation`, [[40-Interfaces/net.minecraft.util.filefix.operations.FileFixOperations|FileFixOperations]], `GroupMove`, `ModifyContent`, `Move`, `RegexMove`, `package-info`

### `net.minecraft.util.filefix.virtualfilesystem` (9 top-level)

`CopyOnWriteFSPath`, `CopyOnWriteFSProvider`, `CopyOnWriteFileStore`, `CopyOnWriteFileSystem`, `DirectoryNode`, `FileMove`, `FileNode`, `Node`, `package-info`

### `net.minecraft.util.filefix.virtualfilesystem.exception` (9 top-level)

`CowFSCreationException`, `CowFSDirectoryNotEmptyException`, `CowFSFileAlreadyExistsException`, `CowFSFileSystemException`, `CowFSIllegalArgumentException`, `CowFSNoSuchFileException`, `CowFSNotDirectoryException`, `CowFSSymlinkException`, `package-info`

