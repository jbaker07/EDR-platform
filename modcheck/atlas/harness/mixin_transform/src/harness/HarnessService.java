package harness;

import java.io.ByteArrayInputStream;
import java.io.File;
import java.io.IOException;
import java.io.InputStream;
import java.net.URL;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Collection;
import java.util.Collections;
import java.util.List;

import org.objectweb.asm.ClassReader;
import org.objectweb.asm.tree.ClassNode;
import org.spongepowered.asm.launch.platform.container.ContainerHandleVirtual;
import org.spongepowered.asm.launch.platform.container.IContainerHandle;
import org.spongepowered.asm.mixin.transformer.IMixinTransformer;
import org.spongepowered.asm.mixin.transformer.IMixinTransformerFactory;
import org.spongepowered.asm.service.IAdviceProvider;
import org.spongepowered.asm.service.IClassBytecodeProvider;
import org.spongepowered.asm.service.IClassProvider;
import org.spongepowered.asm.service.IClassTracker;
import org.spongepowered.asm.service.IFeatureValidator;
import org.spongepowered.asm.service.IMixinAuditTrail;
import org.spongepowered.asm.service.IMixinInternal;
import org.spongepowered.asm.service.ITransformerProvider;
import org.spongepowered.asm.service.ITransformer;
import org.spongepowered.asm.service.MixinServiceAbstract;

/**
 * A minimal Mixin service: enough for the transformer to read class bytes from
 * directories and apply mixins. No game, no loader, no transforming classloader.
 * Modelled on what fabric-loader's Knot service does for the transformer factory.
 */
public class HarnessService extends MixinServiceAbstract
        implements IClassProvider, IClassBytecodeProvider, IClassTracker, IMixinAuditTrail, ITransformerProvider {

    public static volatile IMixinTransformer TRANSFORMER;
    public static final List<String> AUDIT = new ArrayList<>();
    private static final List<Path> DIRS = new ArrayList<>();

    static {
        String dirs = System.getProperty("harness.classdirs", "");
        for (String d : dirs.split(File.pathSeparator)) {
            if (!d.isEmpty()) DIRS.add(Paths.get(d));
        }
    }

    public static byte[] readBytes(String className) throws IOException {
        String rel = className.replace('.', '/') + ".class";
        for (Path dir : DIRS) {
            Path p = dir.resolve(rel);
            if (Files.exists(p)) return Files.readAllBytes(p);
        }
        try (InputStream in = HarnessService.class.getClassLoader().getResourceAsStream(rel)) {
            if (in == null) throw new IOException("class not found in harness dirs or classpath: " + className);
            return in.readAllBytes();
        }
    }

    @Override public String getName() { return "harness"; }
    @Override public boolean isValid() { return true; }

    @Override
    public void offer(IMixinInternal internal) {
        if (internal instanceof IMixinTransformerFactory) {
            TRANSFORMER = ((IMixinTransformerFactory) internal).createTransformer();
        }
        super.offer(internal);
    }

    @Override public IClassProvider getClassProvider() { return this; }
    @Override public IClassBytecodeProvider getBytecodeProvider() { return this; }
    @Override public ITransformerProvider getTransformerProvider() { return this; }
    @Override public IClassTracker getClassTracker() { return this; }
    @Override public IMixinAuditTrail getAuditTrail() { return this; }
    @Override public IFeatureValidator getFeatureValidator() { return null; }
    @Override public IAdviceProvider getAdviceProvider() { return null; }
    @Override public Collection<String> getPlatformAgents() { return Collections.emptyList(); }
    @Override public IContainerHandle getPrimaryContainer() { return new ContainerHandleVirtual(getName()); }

    @Override
    public InputStream getResourceAsStream(String name) {
        for (Path dir : DIRS) {
            Path p = dir.resolve(name);
            if (Files.exists(p)) {
                try { return new ByteArrayInputStream(Files.readAllBytes(p)); } catch (IOException e) { return null; }
            }
        }
        return HarnessService.class.getClassLoader().getResourceAsStream(name);
    }

    // IClassProvider
    @Override public URL[] getClassPath() { return new URL[0]; }
    @Override public Class<?> findClass(String name) throws ClassNotFoundException { return Class.forName(name, false, getClass().getClassLoader()); }
    @Override public Class<?> findClass(String name, boolean initialize) throws ClassNotFoundException { return Class.forName(name, initialize, getClass().getClassLoader()); }
    @Override public Class<?> findAgentClass(String name, boolean initialize) throws ClassNotFoundException { return findClass(name, initialize); }

    // IClassBytecodeProvider
    @Override public ClassNode getClassNode(String name) throws ClassNotFoundException, IOException { return getClassNode(name, true, 0); }
    @Override public ClassNode getClassNode(String name, boolean runTransformers) throws ClassNotFoundException, IOException { return getClassNode(name, runTransformers, 0); }
    @Override
    public ClassNode getClassNode(String name, boolean runTransformers, int readerFlags) throws ClassNotFoundException, IOException {
        byte[] bytes;
        try { bytes = readBytes(name); } catch (IOException e) { throw new ClassNotFoundException(name, e); }
        ClassNode node = new ClassNode();
        new ClassReader(bytes).accept(node, readerFlags);
        return node;
    }

    // IClassTracker
    @Override public void registerInvalidClass(String className) { }
    @Override public boolean isClassLoaded(String className) { return false; }
    @Override public String getClassRestrictions(String className) { return ""; }

    // IMixinAuditTrail
    @Override public void onApply(String className, String mixinName) { AUDIT.add("apply\t" + className + "\t" + mixinName); }
    @Override public void onPostProcess(String className) { AUDIT.add("postprocess\t" + className); }
    @Override public void onGenerate(String className, String generatorName) { AUDIT.add("generate\t" + className + "\t" + generatorName); }

    // ITransformerProvider
    @Override public Collection<ITransformer> getTransformers() { return Collections.emptyList(); }
    @Override public Collection<ITransformer> getDelegatedTransformers() { return Collections.emptyList(); }
    @Override public void addTransformerExclusion(String name) { }
}
