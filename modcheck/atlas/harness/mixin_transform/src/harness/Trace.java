package harness;

import java.util.ArrayList;
import java.util.List;

/** The observation channel: targets and mixin handlers append here; the harness reads it per scenario. */
public final class Trace {
    public static final List<String> LOG = new ArrayList<>();
    private Trace() { }
    public static void log(String s) { LOG.add(s); }
}
