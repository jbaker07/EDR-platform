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
public abstract java.util.Collection<java.lang.String> getProvides()
public abstract net.fabricmc.loader.api.Version getVersion()
public abstract net.fabricmc.loader.api.metadata.ModEnvironment getEnvironment()
public abstract java.util.Collection<net.fabricmc.loader.api.metadata.ModDependency> getDependencies()
public default java.util.Collection<net.fabricmc.loader.api.metadata.ModDependency> getDepends()
public default java.util.Collection<net.fabricmc.loader.api.metadata.ModDependency> getRecommends()
public default java.util.Collection<net.fabricmc.loader.api.metadata.ModDependency> getSuggests()
public default java.util.Collection<net.fabricmc.loader.api.metadata.ModDependency> getConflicts()
public default java.util.Collection<net.fabricmc.loader.api.metadata.ModDependency> getBreaks()
public abstract java.lang.String getName()
public abstract java.lang.String getDescription()
public abstract java.util.Collection<net.fabricmc.loader.api.metadata.Person> getAuthors()
public abstract java.util.Collection<net.fabricmc.loader.api.metadata.Person> getContributors()
public abstract net.fabricmc.loader.api.metadata.ContactInformation getContact()
public abstract java.util.Collection<java.lang.String> getLicense()
public abstract java.util.Optional<java.lang.String> getIconPath(int)
public abstract boolean containsCustomValue(java.lang.String)
public abstract net.fabricmc.loader.api.metadata.CustomValue getCustomValue(java.lang.String)
public abstract java.util.Map<java.lang.String, net.fabricmc.loader.api.metadata.CustomValue> getCustomValues()
public abstract boolean containsCustomElement(java.lang.String)
```
