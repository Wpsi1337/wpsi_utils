{
  description = "Placeholder flake for the toolbox workspace";

  outputs = { self }: {
    packages.x86_64-linux.default = throw "TODO: define Nix build for toolbox";
  };
}
