package fixtures.targets;

import harness.Trace;

public class L {
    public static void go() { new L().run(); }
    public void run() { Trace.log("L.run"); step(); }
    public void step() { Trace.log("L.step"); }
}
