use super::*;

#[derive(Serialize, Deserialize, Debug)]
struct TestClaims {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub iat: u64,
    pub exp: u64,
}

impl JwtClaims for TestClaims {
    fn is_valid(&self) -> bool {
        true
    }
}

impl JwtVerifierByIssuer for TestClaims {
    fn get_iss(&self) -> &str {
        &self.iss
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct TestInviteKeyMockClaims {
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub iat: u64,
    pub exp: u64,
    pub pkh: String,
}

impl JwtClaims for TestInviteKeyMockClaims {
    fn is_valid(&self) -> bool {
        true
    }
}

impl JwtVerifierByIssuer for TestInviteKeyMockClaims {
    fn get_iss(&self) -> &str {
        &self.iss
    }
}

/// Test that we can decode a JWT
#[test]
#[should_panic]
fn jwt_new_should_panic_with_invalid_token() {
    Jwt::<TestClaims>::new("1.2.3").unwrap();
}

/// Test that we can verify a JWT
#[test]
fn jwt_verify_success() {
    let payload = "eyJhbGciOiJFZERTQSIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJkaWQ6a2V5Ono2TWtvZEhad25lVlJTaHRhTGY4SktZa3hwREdwMXZHWm5wR21kQnBYOE0yZXh4SCIsInN1YiI6ImM0NzlmZTVkYzQ2NGU3NzFlNzhiMTkzZDIzOWE2NWI1OGQyNzhjYWQxYzM0YmZiMGI1NzE2ZTViYjUxNDkyOGUiLCJhdWQiOiJ3c3M6Ly9yZWxheS53YWxsZXRjb25uZWN0LmNvbSIsImlhdCI6MTY1NjkxMDA5NywiZXhwIjoxNjU2OTk2NDk3fQ.bAKl1swvwqqV_FgwvD4Bx3Yp987B9gTpZctyBviA-EkAuWc8iI8SyokOjkv9GJESgid4U8Tf2foCgrQp2qrxBA";
    let jwt = Jwt::<TestClaims>::new(payload).unwrap();
    assert!(jwt.verify().is_ok());
}

#[test]
fn jwt_verify_invite_key_success() {
    let payload = "eyJhbGciOiJFZERTQSIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJkaWQ6a2V5Ono2TWt0bXRQY3JWdDZiQkRURFMzVVpOb3lMVGNNTTZjbVV2d1oyU0pVVGJtZlNaRyIsInN1YiI6ImQ0YzkyYWQ0MzA0YWVmOTJhMDM3MWRhZmUzMDFmOGU5YTg2NzQwNGVkM2EwNTM2NGY0NzM2ZDVkMTFhN2FjYzMiLCJhdWQiOiJodHRwczovL3N0YWdpbmcua2V5cy53YWxsZXRjb25uZWN0LmNvbSIsImlhdCI6MTY3NDc0MDQxOSwiZXhwIjoxNzYxMTQwNDE5LCJwa2giOiJkaWQ6cGtoOmVpcDE1NToxOjB4MmNGNjFEMTJhNzA3OGM3OTY1YjQ2NjRlMUM3NEI5ODNmMDNhODNCNiJ9.cjaoYZVsEAPN5oLlyPAHMLEMR7SIFOSLfin3APl8cPslIsx8h0XROA6Iz__dQo228DuE29G_iwaouzZptGgWDw";
    let jwt = Jwt::<TestInviteKeyMockClaims>::new(payload).unwrap();
    assert!(jwt.verify().is_ok());
}

/// Test that we can verify a JWT.
#[test]
fn jwt_verify_fail() {
    let payload = "eyJhbGciOiJFZERTQSIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJkaWQ6a2V5Ono2TWtwSHlDalBqQWs5TmVGWlJuOFJGUVRiaGZ6TEs0Tm5ialJnTGNVVGdzU1RBQyIsInN1YiI6ImY3NjUyYWZiNmRjNGUwN2JmMWNlZTc2NzNkYTExMzI1M2U1NjcwNTJmZGVmZmFjYzdlOTQwNTZmMTQ3NDI1NzMiLCJhdWQiOiJodHRwOi8vMTAuMC4yLjI6ODA4MCIsImlhdCI6MTY3Mzk4NTg1MywiZXhwIjoxNjc0MDcyMjUzLCJwa2giOiJkaWQ6cGtoOmVpcDE1NToxOjB4ZTcyZjk4YWY3YmZlOWEzN2EwNmE2YmY2M2U2OTEyNTYzMTMxN2NlZCJ9.tIx08nEkoJ4M2VZ1uJI6SKSxKhZ31ANa7dXu_b07fXhmKYgujHEyyFk7Ge4OEIEtfH0wrLBOAbnpwEFY2JEwAQ";
    let jwt = Jwt::<TestClaims>::new(payload).unwrap();
    assert!(jwt.verify().is_err());
}
