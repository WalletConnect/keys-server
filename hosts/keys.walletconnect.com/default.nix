{...}: let
  keyserverPort = "8080";
  domain = "keys.walletconnect.com";
  allIpv4 = "0.0.0.0";
  allIpv6 = "[::0]";
in {
  imports = [
    ./hardware-configuration.nix
    ./networking.nix # generated at runtime by nixos-infect
  ];

  networking.firewall = {
    enable = true;
    allowedTCPPorts = [22 80 443];
  };
  security.acme = {
    acceptTerms = true;
    defaults.email = "ops@walletconnect.com";
  };
  boot.cleanTmpDir = true;
  zramSwap.enable = true;
  services.openssh.enable = true;
  system.stateVersion = "22.05";
  users.users.root.openssh.authorizedKeys.keys = [
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIKVet4QtQ6sqKJKBHIO4cZ1sgLSFSoJBL9Ub4JnpnplP github.com/Elyniss.keys"
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIOfN08jC7Rkmnbk2wE1UVuLUalQQU+yYi2017RZ7OcBD sebas@mini"
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIE9642iDnf+zFOhyJABZtFALE+mC7FNRRzzXJ3B5J90O github_action"
  ];
  services.nginx.enable = true;
  services.nginx.virtualHosts."${domain}" = {
    extraConfig = ''
      proxy_set_header Host      $host;
      proxy_set_header X-Real-IP $remote_addr;
      proxy_read_timeout         600;
    '';
    enableACME = true;
    forceSSL = true;
    http2 = true;
    listen = [
      {
        addr = allIpv4;
        port = 443;
        ssl = true;
      }
      {
        addr = allIpv6;
        port = 443;
        ssl = true;
      }
      {
        addr = allIpv4;
        port = 80;
      }
      {
        addr = allIpv6;
        port = 80;
      }
    ];
    locations = {
      "/" = {
        proxyPass = "http://127.0.0.1:${keyserverPort}$request_uri";
        extraConfig = ''
          add_header "Access-Control-Allow-Methods" "GET, POST, PUT, DELETE, OPTIONS";
          add_header "Access-Control-Allow-Headers" "X-Requested-With, Content-Type, Authorization";
          proxy_buffering off;
        '';
      };
    };
  };
}
