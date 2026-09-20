package fixtures.mixins;

import fixtures.target.Target;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.FakeInjector;

/** Carries an annotation in the Mixin namespace the extractor does not know: must be an explicit failure. */
@Mixin(Target.class)
public class UnsupportedMixin {
    @FakeInjector(method = "run")
    private void unknownInjector() { }
}
