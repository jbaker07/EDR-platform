package fixtures.mixins;

import com.llamalad7.mixinextras.injector.ModifyExpressionValue;
import com.llamalad7.mixinextras.injector.ModifyReceiver;
import com.llamalad7.mixinextras.injector.ModifyReturnValue;
import com.llamalad7.mixinextras.injector.v2.WrapWithCondition;
import com.llamalad7.mixinextras.injector.wrapmethod.WrapMethod;
import com.llamalad7.mixinextras.injector.wrapoperation.Operation;
import com.llamalad7.mixinextras.injector.wrapoperation.WrapOperation;
import com.llamalad7.mixinextras.sugar.Local;
import fixtures.target.Target;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;

@Mixin(Target.class)
public class ExtrasMixin {
    @WrapOperation(method = "run", at = @At(value = "INVOKE", target = "Lfixtures/target/Target;helper()V"))
    private void wrapHelper(Target self, Operation<Void> original) { original.call(self); }

    @ModifyReturnValue(method = "compute", at = @At("RETURN"))
    private int modifyReturn(int original) { return original; }

    @ModifyExpressionValue(method = "compute", at = @At(value = "CONSTANT", args = "intValue=2"))
    private int modifyExpr(int original, @Local(argsOnly = true) int x) { return original; }

    @WrapWithCondition(method = "run", at = @At(value = "INVOKE", target = "Lfixtures/target/Target;log()V"))
    private boolean skipLog(Target self) { return true; }

    @WrapMethod(method = "greet")
    private String wrapGreet(String name, Operation<String> original) { return original.call(name); }

    @ModifyReceiver(method = "greet", at = @At(value = "INVOKE", target = "Ljava/lang/String;concat(Ljava/lang/String;)Ljava/lang/String;"))
    private String modifyReceiver(String receiver, String arg) { return receiver; }
}
