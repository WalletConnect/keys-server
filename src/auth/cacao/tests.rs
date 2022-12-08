use crate::auth::cacao::Cacao;

/// Test that we can verify a JWT
#[test]
fn cacao_verify_success() {
    let cacao_serialized = r#"{
      "h": {
        "t": "eip4361"
      },
      "p": {
        "iss": "did:pkh:eip155:1:0xf457f233ab23f863cabc383ebb37b29d8929a17a",
        "domain": "http://10.0.2.2:8080",
        "aud": "http://10.0.2.2:8080",
        "version": "1",
        "nonce": "[B@c3772c7",
        "iat": "2023-01-17T12:15:05+01:00",
        "resources": [
          "did:key:z6MkkG9nM8ksS37sq5mgeoCn5kihLkWANcm9pza5WTkq3tWZ"
        ]
      },
      "s": {
        "t": "eip191",
        "s": "0x1b39982707c70c95f4676e7386052a07b47ecc073b3e9cf47b64b579687d3f68181d48fa9e926ad591ba6954f1a70c597d0772a800bed5fa906384fcd83bcf4f1b"
      }
    } "#;
    let cacao: Cacao = serde_json::from_str(cacao_serialized).unwrap();
    let result = cacao.verify();
    assert!(result.is_ok());
    assert!(result.map_err(|_| false).unwrap());

    let identity_key = cacao.p.identity_key();
    assert!(identity_key.is_ok());
}
