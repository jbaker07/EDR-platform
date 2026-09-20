package fixtures.targets;

import harness.Trace;

public class B {
    public static void go() { new B().run(); }
    public void run() { Trace.log("B.run"); step(); }
    public void step() { Trace.log("B.step"); }
}
