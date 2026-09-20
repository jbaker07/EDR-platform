package fixtures.targets;

import harness.Trace;

public class G {
    public static void go() { new G().run(); }
    public void run() { Trace.log("G.run"); step(); }
    public void step() { Trace.log("G.step"); }
}
