package net.totobirdcreations.voxidianprotocoldatagen;

import net.minecraft.network.NetworkState;
import net.minecraft.network.PacketByteBuf;
import net.minecraft.network.listener.ClientPacketListener;
import net.minecraft.network.state.*;


public enum PacketGroup {
	S2CStatus    ( PacketBound.S2C , PacketStage.Status    , QueryStates         .S2C_FACTORY ),
	S2CLogin     ( PacketBound.S2C , PacketStage.Login     , LoginStates         .S2C_FACTORY ),
	S2CConfig    ( PacketBound.S2C , PacketStage.Config    , ConfigurationStates .S2C_FACTORY ),
	S2CPlay      ( PacketBound.S2C , PacketStage.Play      , PlayStateFactories  .S2C         ),
	C2SHandshake ( PacketBound.C2S , PacketStage.Handshake , HandshakeStates     .C2S_FACTORY ),
	C2SStatus    ( PacketBound.C2S , PacketStage.Status    , QueryStates         .C2S_FACTORY ),
	C2SLogin     ( PacketBound.C2S , PacketStage.Login     , LoginStates         .C2S_FACTORY ),
	C2SConfig    ( PacketBound.C2S , PacketStage.Config    , ConfigurationStates .C2S_FACTORY ),
	C2SPlay      ( PacketBound.C2S , PacketStage.Play      , PlayStateFactories  .C2S         );

	public final PacketBound                bound;
	public final PacketStage                stage;
	public final NetworkState.Factory<?, ?> factory;
	private PacketGroup(
		PacketBound                bound,
		PacketStage                stage,
		NetworkState.Factory<?, ?> factory
	) {
		this.bound   = bound;
		this.stage   = stage;
		this.factory = factory;
	}

	public static enum PacketBound {
		S2C,
		C2S
	}

	public static enum PacketStage {
		Handshake,
		Status,
		Login,
		Config,
		Play
	}

}