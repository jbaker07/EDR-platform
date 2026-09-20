package fixtures.mixins;

import fixtures.target.Target;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Desc;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.Slice;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfo;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfoReturnable;

@Mixin(value = Target.class, priority = 900, remap = false)
public class MultiAtMixin {
    @Inject(method = {"run", "init"}, at = {@At("HEAD"), @At(value = "INVOKE", target = "Lfixtures/target/Target;helper()V", ordinal = 0)}, cancellable = true, require = 1)
    private static void twoPointsTwoMethods(CallbackInfo ci) { }

    @Inject(method = "greet(Ljava/lang/String;)Ljava/lang/String;", at = @At("RETURN"),
            slice = @Slice(from = @At("HEAD"), to = @At(value = "INVOKE", target = "Ljava/lang/String;concat(Ljava/lang/String;)Ljava/lang/String;")))
    private void withDescriptorAndSlice(String name, CallbackInfoReturnable<String> cir) { }

    @Inject(target = @Desc(value = "compute", args = int.class, ret = int.class), at = @At("TAIL"))
    private void descTarget(int x, CallbackInfoReturnable<Integer> cir) { }

    @Inject(method = "lambda$*", at = @At("HEAD"), require = 0)
    private void wildcard(CallbackInfo ci) { }

    @Inject(method = "<init>", at = @At("RETURN"))
    private void ctor(CallbackInfo ci) { }

    @Inject(method = "<clinit>", at = @At("TAIL"))
    private static void clinit(CallbackInfo ci) { }

    @Inject(method = "Lfixtures/target/Target;useField()V", at = @At(value = "FIELD", target = "Lfixtures/target/Target;counter:I", opcode = 179, shift = At.Shift.AFTER, by = 1))
    private void ownerQualified(CallbackInfo ci) { }
}
