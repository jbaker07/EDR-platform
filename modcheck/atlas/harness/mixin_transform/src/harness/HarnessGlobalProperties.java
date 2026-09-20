package harness;

import java.util.HashMap;
import java.util.Map;

import org.spongepowered.asm.service.IGlobalPropertyService;
import org.spongepowered.asm.service.IPropertyKey;

public class HarnessGlobalProperties implements IGlobalPropertyService {
    private final Map<String, Object> props = new HashMap<>();

    static final class Key implements IPropertyKey {
        final String name;
        Key(String name) { this.name = name; }
        @Override public String toString() { return name; }
    }

    @Override public IPropertyKey resolveKey(String name) { return new Key(name); }
    @SuppressWarnings("unchecked") @Override public <T> T getProperty(IPropertyKey key) { return (T) props.get(key.toString()); }
    @Override public void setProperty(IPropertyKey key, Object value) { props.put(key.toString(), value); }
    @SuppressWarnings("unchecked") @Override public <T> T getProperty(IPropertyKey key, T defaultValue) { Object v = props.get(key.toString()); return v == null ? defaultValue : (T) v; }
    @Override public String getPropertyString(IPropertyKey key, String defaultValue) { Object v = props.get(key.toString()); return v == null ? defaultValue : v.toString(); }
}
