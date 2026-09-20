---
type: "interface"
fqcn: "net.minecraft.client.gui.screens.BackupConfirmScreen"
side: "vanilla"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.minecraft.client.gui.screens.BackupConfirmScreen

System: [[20-Systems/net.minecraft.client.gui|net.minecraft.client.gui]]

`class` public; extends `net/minecraft/client/gui/screens/Screen`; implements nothing; identical to the cache jar.

## How Fabric API modules touch this type

| relation | member | descriptor | resolution | operation / site | env | by | evidence |
|---|---|---|---|---|---|---|---|
| calls | `<init>` | `(Ljava/lang/Runnable;Lnet/minecraft/client/gui/screens/BackupConfirmSc` | exact | invokespecial@7 in `DetailedBackupConfirmScreen.<init>` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |
| calls | `init` | `()V` | exact | invokespecial@1 in `DetailedBackupConfirmScreen.init` | unknown | [[30-Mechanisms/fabric-registry-sync-v0|fabric-registry-sync-v0]] | direct_reference |

## Declared members (10 fields, 10 methods, all visibilities)

From the processed jar `97a090f2e55dbcee`. Inherited members are not listed; the resolver walks them (`inherited_exact`).

```
private static final SKIP_AND_JOIN : Lnet/minecraft/network/chat/Component;
public static final BACKUP_AND_JOIN : Lnet/minecraft/network/chat/Component;
private final onCancel : Ljava/lang/Runnable;
protected final onProceed : Lnet/minecraft/client/gui/screens/BackupConfirmScreen$Listener;
private final description : Lnet/minecraft/network/chat/Component;
private final promptForCacheErase : Z
private message : Lnet/minecraft/client/gui/components/MultiLineLabel;
private final confirmation : Lnet/minecraft/network/chat/Component;
protected id : I
private eraseCache : Lnet/minecraft/client/gui/components/Checkbox;
public <init>(Ljava/lang/Runnable;Lnet/minecraft/client/gui/screens/BackupConfirmScreen$Listener;Lnet/minecraft/network/chat/Component;Lnet/minecraft/network/chat/Component;Z)V
public <init>(Ljava/lang/Runnable;Lnet/minecraft/client/gui/screens/BackupConfirmScreen$Listener;Lnet/minecraft/network/chat/Component;Lnet/minecraft/network/chat/Component;Lnet/minecraft/network/chat/Component;Z)V
protected init()V
public extractRenderState(Lnet/minecraft/client/gui/GuiGraphicsExtractor;IIF)V
public shouldCloseOnEsc()Z
public keyPressed(Lnet/minecraft/client/input/KeyEvent;)Z
private synthetic lambda$init$2(Lnet/minecraft/client/gui/components/Button;)V
private synthetic lambda$init$1(Lnet/minecraft/client/gui/components/Button;)V
private synthetic lambda$init$0(Lnet/minecraft/client/gui/components/Button;)V
static <clinit>()V
```
