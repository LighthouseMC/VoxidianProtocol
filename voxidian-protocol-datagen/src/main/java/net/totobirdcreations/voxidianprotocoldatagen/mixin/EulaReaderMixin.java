package net.totobirdcreations.voxidianprotocoldatagen.mixin;

import net.minecraft.server.dedicated.EulaReader;
import org.spongepowered.asm.mixin.Mixin;
import org.spongepowered.asm.mixin.injection.At;
import org.spongepowered.asm.mixin.injection.Inject;
import org.spongepowered.asm.mixin.injection.callback.CallbackInfoReturnable;


@Mixin(EulaReader.class)
class EulaReaderMixin {

	@Inject(at = @At("HEAD"), method = "isEulaAgreedTo", cancellable = true)
	private void isEulaAgreedTo(CallbackInfoReturnable<Boolean> cir) {
		cir.setReturnValue(true);
	}

}