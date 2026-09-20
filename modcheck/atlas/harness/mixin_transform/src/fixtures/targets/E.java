package fixtures.targets;

import harness.Trace;

public class E {
    public static void go() { new E().run(); }
    public void run() { Trace.log("E.run"); step(); }
    public void step() { Trace.log("E.step"); }
}
