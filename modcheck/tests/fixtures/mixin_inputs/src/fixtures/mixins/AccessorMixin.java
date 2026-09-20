package fixtures.mixins;

import fixtures.target.Target;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.gen.Accessor;
import org.spongepowered.asm.mixin.gen.Invoker;

@Mixin(Target.class)
public interface AccessorMixin {
    @Accessor("counter") static int getCounter() { throw new AssertionError(); }
    @Invoker("helper") void callHelper();
}
