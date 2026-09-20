package fixtures.targets;

import harness.Trace;

public class C {
    public static void go() { new C().run(); }
    public void run() { Trace.log("C.run"); step(); }
    public void step() { Trace.log("C.step"); }
}
