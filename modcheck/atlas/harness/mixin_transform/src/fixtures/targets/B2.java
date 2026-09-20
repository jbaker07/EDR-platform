package fixtures.targets;

import harness.Trace;

public class B2 {
    public static void go() { new B2().run(); }
    public void run() { Trace.log("B2.run"); step(); }
    public void step() { Trace.log("B2.step"); }
}
