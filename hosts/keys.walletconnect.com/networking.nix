{ lib, ... }: {
  # This file was populated at runtime with the networking
  # details gathered from the active system.
  networking = {
    nameservers = [ "8.8.8.8"
 ];
    defaultGateway = "159.65.112.1";
    defaultGateway6 = "2a03:b0c0:3:d0::1";
    dhcpcd.enable = false;
    usePredictableInterfaceNames = lib.mkForce false;
    interfaces = {
      eth0 = {
        ipv4.addresses = [
          { address="159.65.123.131"; prefixLength=20; }
{ address="10.19.0.5"; prefixLength=16; }
        ];
        ipv6.addresses = [
          { address="2a03:b0c0:3:d0::1a74:9001"; prefixLength=64; }
{ address="fe80::a4ea:42ff:fe68:ab47"; prefixLength=64; }
        ];
        ipv4.routes = [ { address = "159.65.112.1"; prefixLength = 32; } ];
        ipv6.routes = [ { address = "2a03:b0c0:3:d0::1"; prefixLength = 128; } ];
      };
      
    };
  };
  services.udev.extraRules = ''
    ATTR{address}=="a6:ea:42:68:ab:47", NAME="eth0"
    ATTR{address}=="c6:96:c2:a7:b1:48", NAME="eth1"
  '';
}
