{
	inputs = {
		nixpkgs.url = github:NixOS/nixpkgs/nixos-unstable;
		utils.url = github:numtide/flake-utils;
	};

	outputs = { nixpkgs, utils, ... }: utils.lib.eachDefaultSystem(system: let
		pkgs = import nixpkgs {
			inherit system;
		};
	in {
		devShell = pkgs.mkShell {
			nativeBuildInputs = with pkgs; [
				rustup
			];

			buildInputs = with pkgs; [
				jq
				upx
			];

			shellHook = ''
				rustup toolchain install stable
				rustup toolchain install --profile minimal nightly
				rustup default stable
				rustup component add rust-analyzer
				rustup +nightly component add rustfmt
				rustup update
			'';
		};
	});
}
