package fixtures.mixins;

import fixtures.target.Target;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Constant;
import org.spongepowered.asm.mixin.injection.ModifyArg;
import org.spongepowered.asm.mixin.injection.ModifyConstant;
import org.spongepowered.asm.mixin.injection.ModifyVariable;
import org.spongepowered.asm.mixin.injection.Redirect;

@Mixin(Target.class)
public class RedirectAndModifyMixin {
    @Redirect(method = "run", at = @At(value = "INVOKE", target = "Lfixtures/target/Target;helper()V"))
    private void redirectHelper(Target self) { }

    @ModifyArg(method = "greet", at = @At(value = "INVOKE", target = "Ljava/lang/String;concat(Ljava/lang/String;)Ljava/lang/String;"), index = 0)
    private String modifyArg(String s) { return s; }

    @ModifyVariable(method = "compute", at = @At("HEAD"), ordinal = 0, argsOnly = true)
    private int modifyVar(int x) { return x; }

    @ModifyConstant(method = "compute", constant = @Constant(intValue = 2))
    private int modifyConst(int c) { return c; }
}
