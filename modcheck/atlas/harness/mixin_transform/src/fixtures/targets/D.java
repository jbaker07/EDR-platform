package fixtures.targets;

import harness.Trace;

public class D {
    public static void go() { new D().run(); }
    public void run() { Trace.log("D.run"); step(); }
    public void step() { Trace.log("D.step"); }
}
