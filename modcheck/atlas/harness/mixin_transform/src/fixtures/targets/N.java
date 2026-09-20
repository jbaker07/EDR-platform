package fixtures.targets;

import harness.Trace;

public class N {
    public static void go() { new N().run(); }
    public void run() { Trace.log("N.run"); step(); }
    public void step() { Trace.log("N.step"); }
}
