use crate::auth::cacao::Cacao;

/// Test that we can verify a Cacao.
#[test]
fn cacao_verify_success() {
    let cacao_serialized = r#"{
      "h": {
        "t": "eip4361"
      },
      "p": {
        "iss": "did:pkh:eip155:1:0xB1bad80be351061Db2F726D2dDe28E0Ebbb88D30",
        "domain": "keys.walletconnect.com",
        "aud": "https://keys.walletconnect.com",
        "version": "1",
        "nonce": "2c586f5025cb20094329ccd83684e2b192bebb2a3f83fc91b0b27aa817fd24de",
        "iat": "2023-05-17T14:22:32+02:00",
        "resources": [
          "did:key:z6MkhoV7JnKEFgwai4R1ui14xcPDnqVFZ3a9dUNM3fE3z3Nf"
        ]
      },
      "s": {
        "t": "eip191",
        "s": "0x991f379195564ba1d131c53cc9b3cf13c03e3a8111f502fd40ca12e1d04d98ea58531295c48f852c9c35a938c778f52a2c994f109fc0e94cc4e16f62d41d54371c"
      }
    }"#;
    let cacao: Cacao = serde_json::from_str(cacao_serialized).unwrap();
    let result = cacao.verify();
    assert!(result.is_ok());
    assert!(result.map_err(|_| false).unwrap());

    let identity_key = cacao.p.identity_key();
    assert!(identity_key.is_ok());
}

/// Test that we can verify a Cacao
#[test]
fn cacao_verify_failure() {
    let cacao_serialized = r#"{
      "h": {
        "t": "eip4361"
      },
      "p": {
        "iss": "did:pkh:eip155:1:0xF5dA9A1Aa622903ae73f5eFE46485531913202AF",
        "domain": "keys.walletconnect.com",
        "aud": "https://keys.walletconnect.com",
        "version": "1",
        "nonce": "0d98d4e5d8c19d4cff09cd25f1863bca650d2b4009bd62f04dff7171438c4773",
        "iat": "2023-05-17T14:14:24+02:00",
        "resources": [
          "did:key:z6MkgzojB48jpTcLTatSCRHNpoMRvQbz8r13UJ1KyteHjEu9"
        ]
      },
      "s": {
        "t": "eip191",
        "s": "0x726caf0b066fd857889fa73a8b04cbe249161c37a9342854ec92258a85a91ca5720d6d61afe45c7a54f42373ab1c90d888257637a938af5d9f242adad43b204d1b"
      }
    }"#;
    let cacao: Cacao = serde_json::from_str(cacao_serialized).unwrap();
    let result = cacao.verify();
    assert!(result.is_err());
}
