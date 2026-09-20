---
type: "interface"
fqcn: "net.fabricmc.loader.api.metadata.ModMetadata"
module: "fabric-loader"
---

> [!info] Generated
> Built by `atlas/extract/vault.py` from the records and extracted facts it links to. Do not edit; edit the source and regenerate.

# net.fabricmc.loader.api.metadata.ModMetadata

fabric-loader 0.19.5 -- kind: interface

```java
public abstract java.lang.String getType()
public abstract java.lang.String getId()
public abstract java.util.Collection getProvides()
public abstract net.fabricmc.loader.api.Version getVersion()
public abstract net.fabricmc.loader.api.metadata.ModEnvironment getEnvironment()
public abstract java.util.Collection getDependencies()
public java.util.Collection getDepends()
public java.util.Collection getRecommends()
public java.util.Collection getSuggests()
public java.util.Collection getConflicts()
public java.util.Collection getBreaks()
public abstract java.lang.String getName()
public abstract java.lang.String getDescription()
public abstract java.util.Collection getAuthors()
public abstract java.util.Collection getContributors()
public abstract net.fabricmc.loader.api.metadata.ContactInformation getContact()
public abstract java.util.Collection getLicense()
public abstract java.util.Optional getIconPath(int)
public abstract boolean containsCustomValue(java.lang.String)
public abstract net.fabricmc.loader.api.metadata.CustomValue getCustomValue(java.lang.String)
public abstract java.util.Map getCustomValues()
public abstract boolean containsCustomElement(java.lang.String)
```
