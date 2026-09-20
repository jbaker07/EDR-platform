package fixtures.targets;

import harness.Trace;

public class J {
    public static void go() { new J().run(); }
    public void run() { Trace.log("J.run"); step(); }
    public void step() { Trace.log("J.step"); }
}
