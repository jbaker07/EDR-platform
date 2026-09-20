package harness;

import java.lang.reflect.Method;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.List;

import org.spongepowered.asm.launch.MixinBootstrap;
import org.spongepowered.asm.mixin.MixinEnvironment;
import org.spongepowered.asm.mixin.Mixins;
import org.spongepowered.asm.mixin.transformer.IMixinTransformer;

/**
 * Runs the pinned Mixin transformer over controlled target classes and records
 * what was applied, what failed, and what the transformed code did when run.
 * Usage: Main <out.json> <target class>...
 */
public final class Main {
    private Main() { }

    static final class Loader extends ClassLoader {
        Loader() { super(Main.class.getClassLoader()); }
        Class<?> define(String name, byte[] bytes) { return defineClass(name, bytes, 0, bytes.length); }
    }

    static String json(String s) {
        if (s == null) return "null";
        StringBuilder b = new StringBuilder("\"");
        for (char c : s.toCharArray()) {
            if (c == '"' || c == '\\') b.append('\\').append(c);
            else if (c == '\n') b.append("\\n");
            else if (c == '\t') b.append("\\t");
            else if (c < 0x20) b.append(String.format("\\u%04x", (int) c));
            else b.append(c);
        }
        return b.append('"').toString();
    }

    static String causes(Throwable t) {
        StringBuilder b = new StringBuilder();
        for (Throwable c = t; c != null; c = c.getCause()) {
            if (b.length() > 0) b.append(" <- ");
            b.append(c.getClass().getName()).append(": ").append(c.getMessage());
        }
        return b.toString();
    }

    public static void main(String[] args) throws Exception {
        Path out = Paths.get(args[0]);
        MixinBootstrap.init();
        com.llamalad7.mixinextras.MixinExtrasBootstrap.init();
        Mixins.addConfigurations("moda.mixins.json", "modb.mixins.json");
        MixinEnvironment.getDefaultEnvironment().setSide(MixinEnvironment.Side.SERVER);
        Method gotoPhase = MixinEnvironment.class.getDeclaredMethod("gotoPhase", MixinEnvironment.Phase.class);
        gotoPhase.setAccessible(true);
        gotoPhase.invoke(null, MixinEnvironment.Phase.INIT);
        gotoPhase.invoke(null, MixinEnvironment.Phase.DEFAULT);
        IMixinTransformer transformer = HarnessService.TRANSFORMER;
        if (transformer == null) {
            transformer = (IMixinTransformer) MixinEnvironment.getCurrentEnvironment().getActiveTransformer();
        }
        List<String> results = new ArrayList<>();
        for (int i = 1; i < args.length; i++) {
            results.add(runScenario(transformer, args[i]));
        }
        StringBuilder b = new StringBuilder("{\n \"mixin_version\": ").append(json(MixinBootstrap.VERSION))
                .append(",\n \"java\": ").append(json(System.getProperty("java.version")))
                .append(",\n \"scenarios\": [\n").append(String.join(",\n", results)).append("\n ],\n \"audit\": [");
        for (int i = 0; i < HarnessService.AUDIT.size(); i++) {
            if (i > 0) b.append(", ");
            b.append(json(HarnessService.AUDIT.get(i)));
        }
        b.append("]\n}\n");
        Files.write(out, b.toString().getBytes(StandardCharsets.UTF_8));
        System.out.println("wrote " + out);
    }

    static String runScenario(IMixinTransformer transformer, String target) {
        String transformError = null, runError = null;
        boolean changed = false;
        int auditBefore = HarnessService.AUDIT.size();
        Trace.LOG.clear();
        byte[] transformed = null;
        try {
            byte[] original = HarnessService.readBytes(target);
            transformed = transformer.transformClassBytes(target, target, original);
            changed = !java.util.Arrays.equals(original, transformed);
        } catch (Throwable t) {
            transformError = causes(t);
        }
        if (transformed != null) {
            try {
                Class<?> cls = new Loader().define(target, transformed);
                cls.getMethod("go").invoke(null);
            } catch (Throwable t) {
                runError = causes(t instanceof java.lang.reflect.InvocationTargetException ? t.getCause() : t);
            }
        }
        StringBuilder b = new StringBuilder("  {\"target\": ").append(json(target))
                .append(", \"transformed\": ").append(transformed != null)
                .append(", \"bytes_changed\": ").append(changed)
                .append(", \"transform_error\": ").append(json(transformError))
                .append(", \"run_error\": ").append(json(runError))
                .append(", \"applied\": [");
        List<String> applied = HarnessService.AUDIT.subList(auditBefore, HarnessService.AUDIT.size());
        for (int i = 0; i < applied.size(); i++) {
            if (i > 0) b.append(", ");
            b.append(json(applied.get(i)));
        }
        b.append("], \"trace\": [");
        for (int i = 0; i < Trace.LOG.size(); i++) {
            if (i > 0) b.append(", ");
            b.append(json(Trace.LOG.get(i)));
        }
        return b.append("]}").toString();
    }
}
