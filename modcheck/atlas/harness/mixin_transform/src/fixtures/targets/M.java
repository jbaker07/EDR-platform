package fixtures.targets;

import harness.Trace;

public class M {
    public static void go() { new M().run(); }
    public void run() { Trace.log("M.run"); step(); }
    public void step() { Trace.log("M.step"); }
}
