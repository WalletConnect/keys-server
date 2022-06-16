{...}: {
  imports = [
    ./hardware-configuration.nix
    ./networking.nix # generated at runtime by nixos-infect
  ];

  networking.firewall.allowedTCPPorts = [8080];
  boot.cleanTmpDir = true;
  zramSwap.enable = true;
  services.openssh.enable = true;
  system.stateVersion = "22.05";
  users.users.root.openssh.authorizedKeys.keys = [
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIKVet4QtQ6sqKJKBHIO4cZ1sgLSFSoJBL9Ub4JnpnplP github.com/Elyniss.keys"
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIOfN08jC7Rkmnbk2wE1UVuLUalQQU+yYi2017RZ7OcBD sebas@mini"
  ];
}
