package fixtures.targets;

import harness.Trace;

public class K {
    public static void go() { new K().run(); }
    public void run() { Trace.log("K.run"); step(); }
    public void step() { Trace.log("K.step"); }
}
