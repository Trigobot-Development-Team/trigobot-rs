{ craneLib, ... }:
craneLib.buildPackage {
  src = craneLib.cleanCargoSource (craneLib.path ./.);

  # Add extra inputs here or any other derivation settings
  # doCheck = true;
  # buildInputs = [];
  # nativeBuildInputs = [];

}
