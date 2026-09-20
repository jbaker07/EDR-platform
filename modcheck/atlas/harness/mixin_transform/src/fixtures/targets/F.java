package fixtures.targets;

import harness.Trace;

public class F {
    public static void go() { new F().run(); }
    public void run() { Trace.log("F.run"); step(); }
    public void step() { Trace.log("F.step"); }
}
