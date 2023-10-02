# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -
## 1.12.0 - 2023-10-02
#### Bug Fixes
- switch to `central` Grafana instance (#139) - (c458b45) - Xavier Basty
- syntax for `used_version` task - (a5a9bb6) - Xavier Basty
- force CI to run the "display version" task - (e1e8f26) - Xavier Basty
- don't redeploy app when deploying infra in CD - (19e7991) - Xavier Basty
#### Features
- geo-blocking (#141) - (ce83b63) - Xavier Basty
- add CloudWatch alarms and alarms forwarding to BetterStack (#131) - (b87566b) - Xavier Basty
#### Miscellaneous Chores
- set DocDB available memory threshold to 3GiB (#136) - (2e15a6b) - Xavier Basty
- display actual version passed to the CD (#133) - (08136b5) - Xavier Basty

- - -

## 1.11.0 - 2023-09-07
#### Bug Fixes
- display metric increase for Prometheus (#128) - (1888b9c) - Xavier Basty
- remove `rate` from Prometheus queries (#126) - (b801252) - Xavier Basty
- mask docker password in CI logs - (0f0700b) - Xavier Basty
#### Features
- Allow taking identity key from audience (#130) - (f10ddcc) - Szymon Rząd

- - -

## 1.10.0 - 2023-08-30
#### Bug Fixes
- enable metrics export in ECS (#124) - (db61e22) - Xavier Basty
- CI parameters in `publish` flow - (f89432d) - Xavier Basty
- change the port of the otel collector sidecar (#122) - (20e9766) - Xavier Basty
- remove unnecessary `add-mask` commands - (58ef8ea) - Xavier Basty
- CI parameters in PR flow - (854daa6) - Xavier Basty
- get deployed version in infra only release - (2d3fb88) - Xavier Basty
- hide Grafana key in CI logs - (4bd9326) - Xavier Basty
- CD call in release workflow - (06424db) - Xavier Basty
#### Features
- change log level (#120) - (d75749e) - Szymon Rząd
- move health-check to validate composite workflow for extensibility - (5b2d3de) - Xavier Basty
- add prometheus panels to grafana (#118) - (9e99609) - Xavier Basty
- add HTTP metrics to Grafana (#116) - (df913aa) - Xavier Basty
#### Miscellaneous Chores
- switch to shared action for `update-version` - (1bfdf87) - Xavier Basty
- - -

## 1.9.0 - 2023-08-24
#### Bug Fixes
- run condition for `check-linked-issues` - (a5c0711) - Xavier Basty
- update `Cargo.lock` during releases - (ca7056e) - Xavier Basty
- fix jobs dependencies in deploy dispatch flow - (7d30550) - Xavier Basty
#### Features
- Add health-check for prod in CD - (870f43c) - Xavier Basty
#### Miscellaneous Chores
- rename `keyserver` to `keys-server` in strings (#115) - (5798196) - Xavier Basty
- merge `check linked issues` into PR flow - (b75abcb) - Xavier Basty
- - -

## 1.8.6 - 2023-08-23
#### Bug Fixes
- bump custom actions to 2.1.4 - (59ea1ed) - Xavier Basty
- - -

## 1.8.5 - 2023-08-23
#### Bug Fixes
- ECS deployment - (bbdf333) - Xavier Basty
- - -

## 1.8.4 - 2023-08-23
#### Bug Fixes
- ECS task name - (6ff9982) - Xavier Basty
- - -

## 1.8.3 - 2023-08-23
#### Bug Fixes
- add github token to `protoc` action to avoid rate limits - (fd8d321) - Xavier Basty
- deployment environments - (acc991c) - Xavier Basty
- - -

## 1.8.2 - 2023-08-23
#### Bug Fixes
- ECR repository name - (69791a6) - Xavier Basty
- - -

## 1.8.1 - 2023-08-23
#### Bug Fixes
- conditions in CI - (2b57f8a) - Xavier Basty
- update version usage (#112) - (c7030b6) - Xavier Basty
- CI titles (#111) - (4b51f21) - Xavier Basty
- publish flow (#110) - (2f72afb) - Xavier Basty
- release flows in CI (#109) - (defe4bc) - Xavier Basty
- - -

## 1.8.0 - 2023-08-22
#### Bug Fixes
- **(hotfix)** migrate to dedicated VPC (#94) - (f27ce74) - Szymon Rząd
- **(o11y)** alerts miss service name - (fddd150) - Derek
- trigger release on push to `master` instead of `main` (#105) - (5d617cc) - Xavier Basty
- CI permissions (#104) - (f073d87) - Xavier Basty
- CI permissions (#103) - (b220a43) - Xavier Basty
- use new VPC in docdb (#98) - (7128c0a) - Xavier Basty
- add image version parameter to dispatch workflow event (#90) - (1a8d65d) - Xavier Basty
- remove redundant project actions (#86) - (310c55e) - Xavier Basty
#### Features
- Update CI PAT - (5e08a90) - Szymon Rząd
- Add logging and metrics (#100) - (9c53b33) - Szymon Rząd
- switch to keyserver aws account (#99) - (8cbe136) - Xavier Basty
- tag ECS tasks - (a7b1cb8) - Derek
- add project issues workflow, update project id (#83) - (56812f5) - Xavier Basty
- update Grafana notification channel - (9fc1246) - Derek
- upgrade to latest Grafana - (08d656e) - Derek
#### Miscellaneous Chores
- Grammar - (89bab01) - szymon
- Restore old update version checkout - (c5d2a27) - szymon
- Grammar - (8e56970) - Szymon Rząd
- Grammar (#108) - (66cd3df) - Szymon Rząd
- Update latest release action - (21f3695) - Szymon Rząd
- - -

## 1.7.0 - 2023-05-17
#### Features
- Add read permission GITHUB_TOKEN (#79) - (b370a5f) - Szymon Rząd
- - -

## 1.6.0 - 2023-05-17
#### Bug Fixes
- Fix CAIP122 message generation (#78) - (464746d) - Szymon Rząd
- trigger deploy from CI (#77) - (df4c945) - Xavier Basty
- use lb arn suffix in grafana dashboard (#69) - (0e0ec6d) - Xavier Basty
#### Features
- move state management to TF Cloud (#76) - (3bf3c10) - Xavier Basty
- add autoscaling (#75) - (8048d2c) - Xavier Basty
- add Grafana dashboard (#68) - (6355c90) - Xavier Basty
- - -

## 1.5.1 - 2023-04-12
#### Bug Fixes
- Add second ecs instance (#60) - (8a2019f) - Szymon Rząd
#### Revert
- "feat: Ensure accounts stored as lowercase (#53)" (#64) - (6e01291) - Szymon Rząd
- - -

## 1.5.0 - 2023-03-31
#### Features
- Add act to unregister identity (#57) - (8bf0e36) - Szymon Rząd
- - -

## 1.4.1 - 2023-03-30
#### Bug Fixes
- Fix empty duplicates on identity keys - (21d1516) - szymon
- - -

## 1.4.0 - 2023-03-29
#### Features
- Ensure invite keys are in did:key format (#56) - (076bdb5) - xDarksome
- - -

## 1.3.0 - 2023-03-15
#### Features
- Adjust caip10 account lenght (#54) - (eb6b2f8) - Szymon Rząd
- Ensure accounts stored as lowercase (#53) - (df9a806) - Szymon Rząd
- - -

## 1.2.0 - 2023-02-26
#### Bug Fixes
- Do not kick-off cd on ci - (1f03d28) - szymon
#### Features
- Unregister Identity with did-jwt (#52) - (bc0f5a5) - Szymon Rząd
- - -

## 1.1.1 - 2023-02-16
#### Bug Fixes
- Checkout at tag - (4c28512) - szymon
- checkout at tag - (8e0a2cd) - szymon
- Add missing image_version var - (60a6fb5) - szymon
- Add missing image_version var - (3004220) - szymon
- Dont run ci.yml on version bump commits - (3c8ec25) - szymon
#### Miscellaneous Chores
- Update Readme - (cceb07f) - szymon
- - -

## 1.1.0 - 2023-02-15
#### Bug Fixes
- Fix release - (9de2e16) - szymon
#### Features
- **(ci)** Add image tagging and release (#50) - (36551c0) - Szymon Rząd
- Remove publish step from ci.yml - (7dd729c) - Szymon Rząd
- Add PAT - (c051ab9) - szymon
- - -

Changelog generated by [cocogitto](https://github.com/cocogitto/cocogitto).